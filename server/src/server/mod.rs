pub mod messages;
pub(crate) mod rest;

use std::net::SocketAddr;
use axum::{Error, Extension};
use axum::extract::{ConnectInfo, WebSocketUpgrade};
use axum::extract::ws::Message::{Binary, Text};
use axum::extract::ws::WebSocket;
use axum::http::StatusCode;
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
    let subprotocols = request.headers().get("Sec-WebSocket-Protocol").and_then(|hv| {
        Some(hv.to_str().ok().unwrap_or_default())
    });
    let user = request.extensions().get::<User>().cloned().expect("User not found");
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    let (tx, rx) = tokio::sync::mpsc::channel(100);
    state.write().await.packet_queue.insert(user.id, tx);
    println!("`{user_agent}` at {addr} connected.");
    let response = ws.on_upgrade(move |socket| subscribe_chat(state, rx, socket, addr));
    // The following is necessary for Chromium-based browsers
    return (StatusCode::SWITCHING_PROTOCOLS, [("Sec-WebSocket-Protocol", "Token")], response);
}

pub async fn subscribe_chat(application: SharedState, mut rx: Receiver<Vec<u8>>, mut ws: WebSocket, addr: SocketAddr) {
    let send_task = tokio::spawn(async move {
        while let Some(bytes) = rx.recv().await {
            ws.send(Binary(bytes)).await.unwrap();
        }
    });

    println!("Disconnected!");
}