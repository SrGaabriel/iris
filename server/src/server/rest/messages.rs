use axum::{debug_handler, Extension, Json};
use axum::body::Body;
use axum::extract::Path;
use axum::http::{Request, StatusCode};
use diesel::{debug_query, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, Table};
use diesel::sql_types::BigInt;
use diesel::prelude::*;
use futures_util::FutureExt;
use http_body_util::BodyExt;
use serde::Deserialize;

use crate::entity::message::{CompleteMessage, Message};
use crate::entity::message::messages::{content, edited, id as messageId, user_id};
use crate::entity::message::messages::dsl::messages as messagesTable;
use crate::entity::message::messages::dsl::messages;
use crate::entity::user::User;
use crate::server::messages::{MessageCreated, MessageDeleted, MessageEdited};
use crate::server::rest::{CompletePrivateMessage, error, IrisResponse, IterablePrivateMessage, no_content, ok, PrivateMessage};
use crate::SharedState;

#[debug_handler]
pub async fn create_message(
    Path(contact_id): Path<i64>,
    Extension(state): Extension<SharedState>,
    request: Request<Body>
) -> IrisResponse<CompletePrivateMessage> {
    let user = request.extensions().get::<User>().cloned().expect("User not found");
    let message = Json::from_bytes(request.into_body().collect().await.unwrap().to_bytes().as_ref());
    if message.is_err() {
        return error(StatusCode::BAD_REQUEST, "Invalid message")
    }
    let message: MessageCreationRequest = message.unwrap().0;

    let mut state = state.write().await;

    let reply_message: Option<PrivateMessage> = if let Some(reply) = message.reply_to {
        let query = messages
            .filter(messageId.eq(reply))
            .get_result::<Message>(&mut state.database).optional();

        if query.is_err() {
            return error(StatusCode::NOT_FOUND, "Reply not found");
        }
        let result = query.unwrap();
        if result.is_none() {
            return error(StatusCode::NOT_FOUND, "Reply not found");
        }

        Some(PrivateMessage::from(&result.unwrap()))
    } else {
        None
    };

    let id: i64 = { state.snowflake_issuer.generate().value() as i64 };
    let new_message = Message {
        id,
        user_id: user.id,
        content: message.content.clone(),
        context: contact_id,
        context_type: 0,
        reception_status: 0,
        edited: false,
        reply_to: message.reply_to
    };

    let inserted_message = diesel::insert_into(messages)
        .values(&new_message)
        .get_result::<Message>(&mut state.database)
        .expect("Failed to insert message");

    let message = MessageCreated {
        id: inserted_message.id,
        content: inserted_message.content.clone(),
        user_id: inserted_message.user_id,
        context: inserted_message.context,
        reply_to: inserted_message.reply_to
    };
    if let Some(tx) = state.packet_queue.get(&user.id) {
        tx.send(Box::new(message.clone())).then(|result| {
            if let Err(e) = result {
                eprintln!("Failed to send message: {:?}", e);
            }
            futures_util::future::ready(())
        }).await;
    }

    if let Some(tx) = state.packet_queue.get(&contact_id) {
        tx.send(Box::new(message)).then(|result| {
            if let Err(e) = result {
                eprintln!("Failed to send message: {:?}", e);
            }
            futures_util::future::ready(())
        }).await;
    }

    ok(CompletePrivateMessage::with_reply(&inserted_message, reply_message))
}

// This method will get the messages between the user and the specified contact
pub async fn get_messages(
    Path(contact_id): Path<i64>,
    Extension(state): Extension<SharedState>,
    request: Request<Body>
) -> IrisResponse<Vec<IterablePrivateMessage>> {
    let user = request.extensions().get::<User>().cloned().expect("User not found");

    let connection = &mut state.write().await.database;
    let query = r#"
SELECT
    messages.id,
    messages.user_id,
    messages.content,
    messages.context,
    messages.reception_status,
    messages.edited,
    messages.reply_to,
    COALESCE(
        json_agg(
            json_build_object(
                'count', reactions.reaction_count,
                'me', CASE WHEN reaction_users.user_id = $1 THEN TRUE ELSE FALSE END,
                'emoji', reactions.emoji,
				'reaction', reactions.reaction_id
            )
        ) FILTER (WHERE reactions.message_id IS NOT NULL),
        '[]'
    ) AS reactions
FROM
    messages
LEFT JOIN (
    SELECT
		reactions.reaction_id,
        reactions.message_id,
        reactions.emoji,
        COUNT(reactions.reaction_id) AS reaction_count
    FROM
        reactions
    GROUP BY
        reactions.reaction_id, reactions.message_id, reactions.emoji
) AS reactions ON reactions.message_id = messages.id
LEFT JOIN reaction_users ON reactions.reaction_id = reaction_users.reaction_id
WHERE
    messages.context_type = 0
    AND (
        (messages.user_id = $1 AND messages.context = $2)
        OR (messages.user_id = $2 AND messages.context = $1)
    )
GROUP BY
    messages.id
ORDER BY
    messages.id DESC;
    "#;
    let query = diesel::sql_query(query)
        .bind::<BigInt, _>(user.id)
        .bind::<BigInt, _>(contact_id);
    let bilateral_messages = query.load::<CompleteMessage>(connection).expect("Error loading messages");

    ok(bilateral_messages.iter().map(|m| {
        IterablePrivateMessage {
            id: m.id,
            user_id: m.user_id,
            content: m.content.clone(),
            context: m.context,
            receipt: m.reception_status,
            edited: m.edited,
            reply_to: m.reply_to,
            reactions: serde_json::from_str(&m.reactions).unwrap()
        }
    }).collect())
}

pub async fn edit_message(
    Path(message_id): Path<i64>,
    Extension(state): Extension<SharedState>,
    request: Request<Body>
) -> IrisResponse<PrivateMessage> {
    let user = request.extensions().get::<User>().cloned().expect("User not found");
    let message = Json::from_bytes(request.into_body().collect().await.unwrap().to_bytes().as_ref());
    if message.is_err() {
        return error(StatusCode::BAD_REQUEST, "Invalid message")
    }
    let message: Json<MessageCreationRequest> = message.unwrap();

    let query = messages
        .filter(messageId.eq(message_id))
        .filter(user_id.eq(user.id));

    // now we set both the content and the edited flag to true
    let new_content = message.0.content;
    let state = &mut state.write().await;
    let message = diesel::update(query)
        .set((content.eq(new_content.clone()), edited.eq(true)))
        .returning(messagesTable::all_columns())
        .get_result::<Message>(&mut state.database);

    if message.is_err() {
        return error(StatusCode::NOT_FOUND, "Message not found");
    }
    let message = message.unwrap();

    if let Some(context_tx) = state.packet_queue.get(&message.context) {
        let packet = MessageEdited {
            message_id: message.id,
            new_content,
            editor_id: user.id,
            context_id: message.context
        };
        context_tx.send(Box::new(packet)).then(|result| {
            if let Err(e) = result {
                eprintln!("Failed to send message: {:?}", e);
            }
            futures_util::future::ready(())
        }).await;
    }

    ok(PrivateMessage::from(&message))
}

pub async fn delete_message(
    Path(message_id): Path<i64>,
    Extension(state): Extension<SharedState>,
    request: Request<Body>
) -> IrisResponse<()> {
    let user = request.extensions().get::<User>().cloned().expect("User not found");

    let state = &mut state.write().await;
    let query = messages
        .filter(messageId.eq(message_id))
        .filter(user_id.eq(user.id));
    let deleted = diesel::delete(query).returning(messagesTable::all_columns()).get_result::<Message>(&mut state.database);

    if deleted.is_err() {
        return error(StatusCode::NOT_FOUND, "Message not found");
    }
    let message = deleted.unwrap();

    if let Some(context_tx) = state.packet_queue.get(&message.context) {
        let packet = MessageDeleted { message_id, context_id: message.context };
        context_tx.send(Box::new(packet)).then(|result| {
            if let Err(e) = result {
                eprintln!("Failed to send message: {:?}", e);
            }
            futures_util::future::ready(())
        }).await;
    }

    no_content()
}

#[derive(Deserialize)]
pub struct MessageCreationRequest {
    pub content: String,
    #[serde(default)]
    pub reply_to: Option<i64>
}