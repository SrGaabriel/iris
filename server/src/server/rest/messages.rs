use axum::{debug_handler, Extension, Json};
use axum::body::Body;
use axum::extract::Path;
use axum::http::{Request, StatusCode};
use diesel::{debug_query, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, sql_query, Table};
use diesel::dsl::{exists};
use diesel::pg::Pg;
use diesel::sql_types::{BigInt, Bool, Nullable, SmallInt, Text};
use diesel::prelude::*;
use futures_util::FutureExt;
use http_body_util::BodyExt;
use serde::Deserialize;
use crate::schema::channels::channel_members::dsl::channel_members;
use crate::schema::ctes::select_messages_from;
use crate::schema::messages::{CompleteMessage, Message};
use crate::schema::messages::messages::{content, channel_id as messageChannelId, edited, message_id as messageId, user_id};
use crate::schema::messages::messages::dsl::messages as messagesTable;
use crate::schema::messages::messages::dsl::messages;
use crate::schema::users::User;
use crate::server::gateway::context::{send_packet_to_channel, send_packet_to_context};
use crate::server::gateway::messages::{MessageCreated, MessageDeleted, MessageEdited};
use crate::server::rest::{error, IrisResponse, MessageObject, no_content, ok, StandardUser};
use crate::SharedState;

#[debug_handler]
pub async fn create_message(
    Path(channel_id): Path<i64>,
    Extension(state): Extension<SharedState>,
    request: Request<Body>
) -> IrisResponse<MessageObject> {
    let user = request.extensions().get::<User>().cloned().expect("User not found");
    let message = Json::from_bytes(request.into_body().collect().await.unwrap().to_bytes().as_ref());
    if message.is_err() {
        return error(StatusCode::BAD_REQUEST, "Invalid message")
    }
    let message: MessageCreationRequest = message.unwrap().0;

    let mut state = state.write().await;

    if let Some(reply) = message.reply_to {
        let query = diesel::select(exists(
            messages
                .filter(messageChannelId.eq(channel_id))
                .filter(messageId.eq(reply))
        )).get_result::<bool>(&mut state.database);

        if query.is_err() {
            return error(StatusCode::NOT_FOUND, "Reply not found");
        }
        let result = query.unwrap();
        if !result {
            return error(StatusCode::NOT_FOUND, "Reply not found");
        }
    }

    let id: i64 = { state.snowflake_issuer.generate().value() as i64 };
    let query = sql_query(select_messages_from(
        r#"
        INSERT INTO messages (user_id, message_id, content, channel_id, reception_status, edited, reply_to)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING *
        "#
    ))
        .bind::<BigInt, _>(user.user_id)
        .bind::<BigInt, _>(id)
        .bind::<Text, _>(message.content.clone())
        .bind::<BigInt, _>(channel_id)
        .bind::<SmallInt, _>(0)
        .bind::<Bool, _>(false)
        .bind::<Nullable<BigInt>, _>(message.reply_to)
        .get_result::<CompleteMessage>(&mut state.database);

    if query.is_err() {
        query.unwrap();
        return error(StatusCode::INTERNAL_SERVER_ERROR, "Error inserting message");
    }
    let inserted_message = query.unwrap();

    let message = MessageObject {
        id: inserted_message.message_id,
        user_id: inserted_message.user_id,
        content: inserted_message.content,
        channel_id: inserted_message.channel_id,
        receipt: inserted_message.reception_status,
        edited: inserted_message.edited,
        author: StandardUser {
            id: inserted_message.user_id,
            name: inserted_message.author_name.clone(),
            username: inserted_message.author_username.clone()
        },
        reply_to: inserted_message.reply_to,
        reactions: vec![]
    };
    send_packet_to_channel(state, channel_id, || Box::new(MessageCreated {
        message: message.clone()
    })).await;

    ok(message)
}

// This method will get the messages between the user and the specified contact
pub async fn get_messages(
    Path(channel_id): Path<i64>,
    Extension(state): Extension<SharedState>,
    request: Request<Body>
) -> IrisResponse<Vec<MessageObject>> {
    let user = request.extensions().get::<User>().cloned().expect("User not found");

    let connection = &mut state.write().await.database;

    {
        let is_member = diesel::select(
            exists(
                channel_members
                    .filter(crate::schema::channels::channel_members::user_id.eq(user.user_id))
                    .filter(crate::schema::channels::channel_members::channel_id.eq(channel_id))
            )
        ).get_result::<bool>(connection).expect("Error checking if user is a member");

        if !is_member {
            return error(StatusCode::FORBIDDEN, "You are not a member of this channel");
        }
    }

    let query = sql_query(select_messages_from(
        "SELECT * FROM messages WHERE channel_id = $2"
    ))
        .bind::<BigInt, _>(user.user_id)
        .bind::<BigInt, _>(channel_id);
    let bilateral_messages = query.load::<CompleteMessage>(connection).expect("Error loading messages");

    ok(bilateral_messages.iter().map(|m| {
        MessageObject {
            id: m.message_id,
            user_id: m.user_id,
            content: m.content.clone(),
            channel_id: m.channel_id,
            receipt: m.reception_status,
            edited: m.edited,
            author: StandardUser {
                id: m.user_id,
                name: m.author_name.clone(),
                username: m.author_username.clone()
            },
            reply_to: m.reply_to,
            reactions: serde_json::from_str(&m.reactions).unwrap()
        }
    }).collect())
}

pub async fn edit_message(
    Path((channel_id, message_id)): Path<(i64, i64)>,
    Extension(state): Extension<SharedState>,
    request: Request<Body>
) -> IrisResponse<MessageObject> {
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

    let new_content = message.0.content;
    let mut state = state.write().await;
    let message = diesel::sql_query(select_messages_from(
        "UPDATE messages SET content = $2, edited = true WHERE channel_id = $3 AND message_id = $4 and user_id=$1 RETURNING *"
    ))
        .bind::<BigInt, _>(user.user_id)
        .bind::<Text, _>(new_content.clone())
        .bind::<BigInt, _>(channel_id)
        .bind::<BigInt, _>(message_id)
        .get_result::<CompleteMessage>(&mut state.database);

    if message.is_err() {
        message.unwrap();
        return error(StatusCode::NOT_FOUND, "Message not found");
    }
    let message = message.unwrap();
    let object = MessageObject {
        id: message.message_id,
        user_id: message.user_id,
        content: message.content,
        channel_id: message.channel_id,
        receipt: message.reception_status,
        edited: message.edited,
        author: StandardUser {
            id: message.user_id,
            name: message.author_name.clone(),
            username: message.author_username.clone()
        },
        reply_to: message.reply_to,
        reactions: serde_json::from_str(&message.reactions).unwrap()
    };

    send_packet_to_channel(state, channel_id, || Box::new(MessageEdited {
        new_content: new_content.clone(),
        editor_id: user.user_id,
        message_id: message.message_id,
        channel_id: message.channel_id,
    })).await;

    ok(object)
}

pub async fn delete_message(
    Path((channel_id, message_id)): Path<(i64, i64)>,
    Extension(state): Extension<SharedState>,
    request: Request<Body>
) -> IrisResponse<()> {
    let user = request.extensions().get::<User>().cloned().expect("User not found");

    let mut state = state.write().await;
    let query = messages
        .filter(messageChannelId.eq(channel_id))
        .filter(messageId.eq(message_id))
        .filter(user_id.eq(user.user_id));
    let deleted = diesel::delete(query).returning(messagesTable::all_columns()).get_result::<Message>(&mut state.database);

    if deleted.is_err() {
        return error(StatusCode::NOT_FOUND, "Message not found");
    }
    let message = deleted.unwrap();

    send_packet_to_channel(state, channel_id, || Box::new(MessageDeleted {
        message_id: message.message_id,
        channel_id: message.channel_id
    })).await;

    no_content()
}

#[derive(Deserialize)]
pub struct MessageCreationRequest {
    pub content: String,
    #[serde(default)]
    pub reply_to: Option<i64>
}