use axum::{debug_handler, Extension, Json};
use axum::body::Body;
use axum::extract::Path;
use axum::http::{Request, StatusCode};
use diesel::{debug_query, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, Table};
use diesel::pg::Pg;
use diesel::sql_types::BigInt;
use diesel::prelude::*;
use futures_util::FutureExt;
use http_body_util::BodyExt;
use serde::Deserialize;

use crate::schema::messages::{CompleteMessage, Message};
use crate::schema::messages::messages::{content, channel_id as messageChannelId, edited, message_id as messageId, user_id};
use crate::schema::messages::messages::dsl::messages as messagesTable;
use crate::schema::messages::messages::dsl::messages;
use crate::schema::users::User;
use crate::server::gateway::context::{send_packet_to_channel, send_packet_to_context};
use crate::server::messages::{MessageCreated, MessageDeleted, MessageEdited};
use crate::server::rest::{CompletePrivateMessage, error, IrisResponse, IterablePrivateMessage, no_content, ok, PrivateMessage};
use crate::SharedState;

#[debug_handler]
pub async fn create_message(
    Path(channel_id): Path<i64>,
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
        message_id: id,
        user_id: user.user_id,
        content: message.content.clone(),
        channel_id,
        reception_status: 0,
        edited: false,
        reply_to: message.reply_to
    };

    let inserted_message = diesel::insert_into(messages)
        .values(&new_message)
        .get_result::<Message>(&mut state.database)
        .expect("Failed to insert message");

    let message = MessageCreated {
        id: inserted_message.message_id,
        content: inserted_message.content.clone(),
        user_id: inserted_message.user_id,
        channel_id: inserted_message.channel_id,
        reply_to: inserted_message.reply_to
    };
    send_packet_to_channel(state, channel_id, || Box::new(message.clone())).await;

    ok(CompletePrivateMessage::with_reply(&inserted_message, reply_message))
}

// This method will get the messages between the user and the specified contact
pub async fn get_messages(
    Path(channel_id): Path<i64>,
    Extension(state): Extension<SharedState>,
    request: Request<Body>
) -> IrisResponse<Vec<IterablePrivateMessage>> {
    let user = request.extensions().get::<User>().cloned().expect("User not found");

    let connection = &mut state.write().await.database;
    let query = r#"
WITH reactions_with_me AS (
    SELECT
        reactions.reaction_id,
        reactions.message_id,
        reactions.emoji,
        reactions.reaction_count,
        bool_or(reaction_users.user_id = $1) AS me
    FROM
        reactions
    LEFT JOIN reaction_users ON reactions.reaction_id = reaction_users.reaction_id
    GROUP BY
        reactions.reaction_id, reactions.message_id, reactions.emoji, reactions.reaction_count
)
SELECT
    messages.message_id,
    messages.user_id,
    messages.content,
    messages.channel_id,
    messages.reception_status,
    messages.edited,
    messages.reply_to,
    COALESCE(
        json_agg(
            json_build_object(
                'reaction_id', reactions_with_me.reaction_id,
                'count', reactions_with_me.reaction_count,
                'me', reactions_with_me.me,
                'emoji', reactions_with_me.emoji
            )
        ) FILTER (WHERE reactions_with_me.message_id IS NOT NULL AND reactions_with_me.reaction_count > 0),
        '[]'
    ) AS reactions
FROM
    messages
LEFT JOIN reactions_with_me ON reactions_with_me.message_id = messages.message_id
WHERE
    messages.channel_id = $2
GROUP BY
    messages.message_id
ORDER BY
    messages.message_id DESC;
    "#;
    let query = diesel::sql_query(query)
        .bind::<BigInt, _>(user.user_id)
        .bind::<BigInt, _>(channel_id);
    let bilateral_messages = query.load::<CompleteMessage>(connection).expect("Error loading messages");

    ok(bilateral_messages.iter().map(|m| {
        IterablePrivateMessage {
            id: m.message_id,
            user_id: m.user_id,
            content: m.content.clone(),
            context: m.channel_id,
            receipt: m.reception_status,
            edited: m.edited,
            reply_to: m.reply_to,
            reactions: serde_json::from_str(&m.reactions).unwrap()
        }
    }).collect())
}

pub async fn edit_message(
    Path((channel_id, message_id)): Path<(i64, i64)>,
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
        .filter(messageChannelId.eq(channel_id))
        .filter(messageId.eq(message_id))
        .filter(user_id.eq(user.user_id));

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

    send_packet_to_context(&mut state.packet_queue, channel_id, Box::new(MessageEdited {
        message_id: message.message_id,
        new_content,
        editor_id: user.user_id,
        context_id: message.channel_id
    })).await;

    ok(PrivateMessage::from(&message))
}

pub async fn delete_message(
    Path((channel_id, message_id)): Path<(i64, i64)>,
    Extension(state): Extension<SharedState>,
    request: Request<Body>
) -> IrisResponse<()> {
    let user = request.extensions().get::<User>().cloned().expect("User not found");

    let state = &mut state.write().await;
    let query = messages
        .filter(messageChannelId.eq(channel_id))
        .filter(messageId.eq(message_id))
        .filter(user_id.eq(user.user_id));
    let deleted = diesel::delete(query).returning(messagesTable::all_columns()).get_result::<Message>(&mut state.database);

    if deleted.is_err() {
        return error(StatusCode::NOT_FOUND, "Message not found");
    }
    let message = deleted.unwrap();

    send_packet_to_context(&mut state.packet_queue, channel_id, Box::new(MessageDeleted {
        message_id: message.message_id,
        context_id: channel_id
    })).await;

    no_content()
}

#[derive(Deserialize)]
pub struct MessageCreationRequest {
    pub content: String,
    #[serde(default)]
    pub reply_to: Option<i64>
}