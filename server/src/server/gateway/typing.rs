use async_trait::async_trait;
use tokio::sync::RwLockWriteGuard;

use crate::AppState;
use crate::entity::user::User;
use crate::server::gateway::GatewayHandler;
use crate::server::messages::{ContactTyping, PacketMessage, PacketStaticId, TypingRequest};
use crate::server::messages::Packet;

pub struct TypingGatewayHandler;

#[async_trait]
impl GatewayHandler for TypingGatewayHandler {
    fn get_id(&self) -> i32 {
        <TypingRequest as PacketStaticId>::get_id()
    }

    async fn handle(&self, user: &User, state: &mut RwLockWriteGuard<AppState>, message: &PacketMessage) {
        let request = TypingRequest::decode_data(&message.data).expect("Failed to decode context read packet");

        let target_tx = state.packet_queue.get(&request.context_id);
        if let Some(tx) = target_tx {
            println!("Sending contact typing...");
            tx.send(Box::new(ContactTyping {
                contact_id: user.id,
            })).await.expect("Failed to send contact typing packet");
        }
    }
}