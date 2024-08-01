use std::net::SocketAddr;

use axum::Extension;
use axum::extract::{ConnectInfo, WebSocketUpgrade};
use axum::extract::ws::Message::Binary;
use axum::extract::ws::WebSocket;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_extra::TypedHeader;
use diesel::{BoolExpressionMethods, QueryDsl, RunQueryDsl};
use diesel::ExpressionMethods;
use futures::{sink::SinkExt, stream::StreamExt};
use prost::Message;
use tokio::sync::mpsc::Receiver;

use crate::schema::users::User;
use crate::server::gateway::Gateway;
use crate::server::messages::{encode_packet_message, Packet, PacketMessage};
use crate::SharedState;

pub mod messages;
pub mod rest;
pub mod gateway;

#[axum::debug_handler]
pub async fn subscribe_chat_handshake(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Extension(state): Extension<SharedState>,
    request: axum::http::Request<axum::body::Body>
) -> impl IntoResponse {
    let _ = request.headers().get("Sec-WebSocket-Protocol").and_then(|hv| {
        Some(hv.to_str().ok().unwrap_or_default())
    });
    let user = request.extensions().get::<User>().cloned().expect("User not found");
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    let (tx, rx) = tokio::sync::mpsc::channel(100);
    {
        state.write().await.packet_queue.insert(user.user_id, tx);
    }
    println!("`{user_agent}` at {addr} connected.");
    let response = ws.on_upgrade(move |socket| subscribe_chat(user, state, rx, socket, addr));
    // The following is necessary for Chromium-based browsers
    return (StatusCode::SWITCHING_PROTOCOLS, [("Sec-WebSocket-Protocol", "Token")], response);
}

pub async fn subscribe_chat(connected_user: User, application: SharedState, mut rx: Receiver<Box<dyn Packet + Send>>, mut ws: WebSocket, addr: SocketAddr) {
    let (mut sender, mut receiver) = ws.split();

    let send_task = tokio::spawn(async move {
        while let Some(bytes) = rx.recv().await {
            sender.send(Binary(encode_packet_message(bytes))).await.unwrap();
        }
    });

    let receive_task = tokio::spawn(async move {
        while let Some(Ok(Binary(binary))) = receiver.next().await {
            let packet = PacketMessage::decode(binary.as_slice()).unwrap();
            let mut state = application.write().await;
            let mut temp_gateway = std::mem::replace(&mut state.gateway, Gateway::new());
            temp_gateway.handle_packet(&connected_user, &mut state, &packet).await;
            state.gateway = temp_gateway;
        }
    });

    println!("Disconnected!");
}