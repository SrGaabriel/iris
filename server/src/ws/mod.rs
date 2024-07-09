pub mod messages;

use std::net::SocketAddr;
use axum::extract::{ConnectInfo, WebSocketUpgrade};
use axum::extract::ws::Message::{Binary, Text};
use axum::extract::ws::WebSocket;
use axum::response::IntoResponse;
use axum_extra::TypedHeader;
use prost::Message;
use crate::ws::messages::TextMessage;

pub async fn subscribe_chat_handshake(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    println!("`{user_agent}` at {addr} connected.");
    ws.on_upgrade(move |socket| subscribe_chat(socket, addr))
}

pub async fn subscribe_chat(mut ws: WebSocket, addr: SocketAddr) {
    let mut receiver_task = tokio::spawn(async move {
        let mut count = 0;
        while let Some(message) = ws.recv().await {
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
            // ws.send(Text("Hello".to_string())).await.unwrap();
        }
    });
}