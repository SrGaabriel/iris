use async_trait::async_trait;
use tokio::sync::RwLockWriteGuard;

use crate::AppState;
use crate::schema::users::User;
use crate::server::gateway::context::send_packet_to_channel;
use crate::server::gateway::GatewayHandler;
use crate::server::gateway::messages::{ChannelTyping, TypingRequest};
use crate::server::messages::{Packet, PacketMessage, PacketStaticId};
use crate::server::rest::StandardUser;

pub struct TypingGatewayHandler;

#[async_trait]
impl GatewayHandler for TypingGatewayHandler {
    fn get_id(&self) -> i32 {
        <TypingRequest as PacketStaticId>::get_id()
    }

    async fn handle(&self, user: &User, state: &mut RwLockWriteGuard<AppState>, message: &PacketMessage) {
        let request: TypingRequest = TypingRequest::decode_data(&message.data).expect("Failed to decode context read packet");

        send_packet_to_channel(
            state,
            request.channel_id,
            || Box::new(ChannelTyping {
                user: StandardUser {
                    id: user.user_id,
                    name: user.name.clone(),
                    username: user.username.clone()
                },
                channel_id: request.channel_id,
            })
        ).await;
    }
}