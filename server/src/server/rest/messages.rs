use axum::{debug_handler, Extension, Json};
use axum::body::Body;
use axum::extract::{FromRequest, Path};
use axum::http::{Request, StatusCode};
use diesel::{BoolExpressionMethods, debug_query, ExpressionMethods, IntoSql, QueryDsl, RunQueryDsl};
use futures_util::FutureExt;
use http_body_util::BodyExt;
use prost::Message as ProstMessage;
use rand::random;
use serde::Deserialize;

use crate::entity::message::Message;
use crate::entity::message::messages::{context, context_type, user_id};
use crate::entity::message::messages::dsl::messages;
use crate::entity::user::User;
use crate::server::messages::TextMessageResponse;
use crate::server::rest::{error, IrisResponse, ok, PrivateMessage};
use crate::SharedState;

#[debug_handler]
pub async fn create_message(
    Path(contact_id): Path<i64>,
    Extension(state): Extension<SharedState>,
    request: Request<Body>
) -> IrisResponse<PrivateMessage> {
    let user = request.extensions().get::<User>().cloned().expect("User not found");
    let message = Json::from_bytes(request.into_body().collect().await.unwrap().to_bytes().as_ref());
    if message.is_err() {
        return error(StatusCode::BAD_REQUEST, "Invalid message")
    }
    let message: Json<MessageCreationRequest> = message.unwrap();

    let mut state = state.write().await;
    let id: i64 = { state.snowflake_issuer.generate().value() as i64 };
    let new_message = Message {
        id,
        user_id: user.id,
        content: message.0.content.clone(),
        context: contact_id,
        context_type: 0,
    };

    let connection = &mut state.database;
    let inserted_message = diesel::insert_into(messages)
        .values(&new_message)
        .get_result::<Message>(connection)
        .expect("Failed to insert message");

    // Now we need to communicate our message to the connected target
    println!("Packet queue: {:?}", state.packet_queue);
    if let Some(tx) = state.packet_queue.get(&contact_id) {
        println!("There is something");
        let response = TextMessageResponse {
            id: inserted_message.id,
            content: inserted_message.content.clone(),
            user_id: inserted_message.user_id,
            context: inserted_message.context,
        };
        tx.send(response.encode_to_vec()).then(|result| {
            if let Err(e) = result {
                eprintln!("Failed to send message: {:?}", e);
            }
            futures_util::future::ready(())
        }).await;
        println!("Sent message to target");
    }

    ok(PrivateMessage::from(&inserted_message))
}

// This method will get the messages between the user and the specified contact
pub async fn get_messages(
    Path(contact_id): Path<i64>,
    Extension(state): Extension<SharedState>,
    request: Request<Body>
) -> IrisResponse<Vec<PrivateMessage>> {
    let user = request.extensions().get::<User>().cloned().expect("User not found");

    let connection = &mut state.write().await.database;
    let query = messages
        .filter(context_type.eq(0))
        .filter(
            (user_id.eq(user.id).and(context.eq(contact_id)))
                .or(user_id.eq(contact_id).and(context.eq(user.id)))
        );
    let bilateral_messages = query.load::<Message>(connection).expect("Error loading messages");

    ok(bilateral_messages.iter().map(|m| PrivateMessage::from(m)).collect())
}

#[derive(Deserialize)]
pub struct MessageCreationRequest {
    pub content: String
}