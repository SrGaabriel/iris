use dashmap::DashMap;
use futures_util::FutureExt;
use tokio::sync::mpsc::Sender;
use tokio::sync::RwLockWriteGuard;
use crate::AppState;
use diesel::{QueryDsl, RunQueryDsl};
use diesel::ExpressionMethods;
use crate::schema::channels::channel_members::dsl::channel_members;
use crate::schema::channels::channel_members::{channel_id as table_channel_id, user_id};
use crate::server::messages::Packet;

#[deprecated]
pub async fn send_packet_to_context(packet_queue: &mut DashMap<i64, Sender<Box<dyn Packet + Send>>>, context: i64, packet: Box<dyn Packet + Send>) {
    println!("Sending packet to context: {}", context);
    if let Some(tx) = packet_queue.get(&context) {
        println!("Context was found, now sending packet");
        tx.send(packet).then(|result| {
            if let Err(e) = result {
                eprintln!("Failed to send message: {:?}", e);
            }
            futures_util::future::ready(())
        }).await;
    }
}

pub async fn send_packet_to_channel<F>(
    mut lock: RwLockWriteGuard<'_, AppState>,
    channel_id: i64,
    packet_fn: F
) where F: Fn() -> Box<dyn Packet + Send> {
    let members: Vec<i64> = {
        let _members = channel_members
            .filter(table_channel_id.eq(channel_id))
            .select(user_id)
            .load::<i64>(&mut lock.database);
        if _members.is_err() {
            return;
        }
        _members.unwrap()
    };
    for member in members {
        send_packet_to_user(&mut lock.packet_queue, member, packet_fn()).await;
    }
}

pub async fn send_packet_to_user(packet_queue: &mut DashMap<i64, Sender<Box<dyn Packet + Send>>>, user: i64, packet: Box<dyn Packet + Send>) {
    println!("Sending packet to user: {}", user);
    if let Some(tx) = packet_queue.get(&user) {
        println!("Context was found, now sending packet");
        tx.send(packet).then(|result| {
            if let Err(e) = result {
                eprintln!("Failed to send message: {:?}", e);
            }
            futures_util::future::ready(())
        }).await;
    }
}