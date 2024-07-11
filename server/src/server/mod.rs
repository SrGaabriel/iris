pub mod messages;
pub(crate) mod rest;

use std::net::SocketAddr;
use axum::{Error, Extension};
use axum::extract::{ConnectInfo, WebSocketUpgrade};
use axum::extract::ws::Message::{Binary, Text};
use axum::extract::ws::WebSocket;
use axum::response::IntoResponse;
use axum_extra::TypedHeader;
use crossbeam::queue::SegQueue;
use prost::Message;
use tokio::sync::mpsc::Receiver;
use crate::entity::user::User;
use crate::SharedState;
use crate::server::messages::TextMessageRequest;

#[axum::debug_handler]
pub async fn subscribe_chat_handshake(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Extension(state): Extension<SharedState>,
    request: axum::http::Request<axum::body::Body>
) -> impl IntoResponse {
    let user = request.extensions().get::<User>().cloned().expect("User not found");
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    let (tx, rx) = tokio::sync::mpsc::channel(100);
    state.write().await.packet_queue.insert(user.id, tx);
    println!("`{user_agent}` at {addr} connected.");
    ws.on_upgrade(move |socket| subscribe_chat(state, rx, socket, addr))
}

pub async fn subscribe_chat(application: SharedState, mut rx: Receiver<Vec<u8>>, mut ws: WebSocket, addr: SocketAddr) {
    // tokio::spawn(async move {
    //     let mut count = 0;
    //     while let Some(message) = ws.recv().await {
    //         if message.is_err() {
    //             break;
    //         }
    //         let original_message = message.unwrap();
    //         count += 1;
    //         let message = TextMessageRequest::decode(&*original_message.clone().into_data()).unwrap();
    //         println!("Received message #{}: {:?}", count, message);
    //
    //         let response = TextMessageRequest {
    //             content: format!("You said: {}", message.content)
    //         };
    //         let mut buf = Vec::new();
    //         response.encode(&mut buf).unwrap();
    //         ws.send(Binary(buf)).await.unwrap();
    //     }
    // });
    tokio::spawn(async move {
        while let Some(bytes) = rx.recv().await {
            ws.send(Binary(bytes)).await.unwrap();
        }
    });
}