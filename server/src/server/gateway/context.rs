use dashmap::DashMap;
use futures_util::FutureExt;
use tokio::sync::mpsc::Sender;
use crate::server::messages::Packet;

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