use std::net::SocketAddr;

use axum::Extension;
use axum::extract::{ConnectInfo, WebSocketUpgrade};
use axum::extract::ws::Message::Binary;
use axum::extract::ws::WebSocket;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_extra::TypedHeader;
use diesel::{BoolExpressionMethods, QueryDsl, RunQueryDsl};
use futures::{sink::SinkExt, stream::StreamExt};
use prost::Message;
use tokio::sync::mpsc::Receiver;
use crate::entity::message::messages::{context, reception_status, user_id};
use crate::entity::message::messages::dsl::messages as messagesTable;
use crate::entity::user::User;
use crate::server::messages::{ContextRead, encode_packet_message, Packet, PacketMessage};
use crate::SharedState;
use diesel::ExpressionMethods;

pub mod messages;
pub mod rest;

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
    let response = ws.on_upgrade(move |socket| subscribe_chat(user.id, state, rx, socket, addr));
    // The following is necessary for Chromium-based browsers
    return (StatusCode::SWITCHING_PROTOCOLS, [("Sec-WebSocket-Protocol", "Token")], response);
}

pub async fn subscribe_chat(connected_user_id: i64, application: SharedState, mut rx: Receiver<Box<dyn Packet + Send>>, mut ws: WebSocket, addr: SocketAddr) {
    let (mut sender, mut receiver) = ws.split();

    let send_task = tokio::spawn(async move {
        while let Some(bytes) = rx.recv().await {
            sender.send(Binary(encode_packet_message(bytes))).await.unwrap();
        }
    });

    let receive_task = tokio::spawn(async move {
        while let Some(Ok(Binary(binary))) = receiver.next().await {
            let packet = PacketMessage::decode(binary.as_slice()).unwrap();
            match packet.id {
                1 => {
                    let request = ContextRead::decode(packet.data.as_slice());
                    if let Ok(request) = request {
                        println!("Received context read request: {:?}", request);
                        println!("Updating reception status... {}, {}", connected_user_id, request.context_id);
                        let query = diesel::update(messagesTable)
                            .filter(context.eq(connected_user_id).and(reception_status.ne(2)).and(user_id.eq(request.context_id)))
                            .set(reception_status.eq(2));
                        println!("{}", diesel::debug_query::<diesel::pg::Pg, _>(&query));
                        let state = &mut application.write().await;
                        query.execute(&mut state.database).unwrap();
                    }
                },
                _ => {
                    println!("Unknown packet ID: {}", packet.id);
                }
            }
        }
    });

    println!("Disconnected!");
}