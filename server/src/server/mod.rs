pub mod messages;
pub(crate) mod rest;

use std::net::SocketAddr;
use axum::{Error, Extension};
use axum::extract::{ConnectInfo, WebSocketUpgrade};
use axum::extract::ws::Message::{Binary, Text};
use axum::extract::ws::WebSocket;
use axum::response::IntoResponse;
use axum_extra::TypedHeader;
use prost::Message;
use crate::entity::user::User;
use crate::SharedState;
use crate::server::messages::TextMessage;

#[axum::debug_handler]
pub async fn subscribe_chat_handshake(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Extension(state): Extension<SharedState>,
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    state.write().unwrap().connections.insert(addr, User {
        id: 0,
        name: String::from("John Doe"),
        username: String::from("Anonymous"),
        email: String::from("john@doe.com"),
        password: String::from("password")
    });
    println!("`{user_agent}` at {addr} connected.");
    ws.on_upgrade(move |socket| subscribe_chat(state, socket, addr))
}

pub async fn subscribe_chat(application: SharedState, mut ws: WebSocket, addr: SocketAddr) {
    let mut receiver_task = tokio::spawn(async move {
        let mut count = 0;
        while let Some(message) = ws.recv().await {
            if message.is_err() {
                break;
            }
            let original_message = message.unwrap();
            count += 1;
            let message = TextMessage::decode(&*original_message.clone().into_data()).unwrap();
            println!("Received message #{}: {:?}", count, message);

            let response = TextMessage {
                content: format!("You said: {}", message.content)
            };
            let mut buf = Vec::new();
            response.encode(&mut buf).unwrap();
            ws.send(Binary(buf)).await.unwrap();
        }
    });
}