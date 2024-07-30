use async_trait::async_trait;
use crate::schema::users::User;
use crate::server::gateway::GatewayHandler;
use crate::server::messages::{ContextRead, MessagesRead, PacketMessage};
use crate::{AppState, SharedState};
use diesel::{BoolExpressionMethods, QueryDsl, RunQueryDsl, ExpressionMethods};
use futures_util::FutureExt;
use tokio::sync::RwLockWriteGuard;
use crate::schema::messages::messages::{message_id as messageId, channel_id as messageChannelId, reception_status, user_id};
use crate::schema::messages::messages::dsl::messages as messagesTable;
use crate::server::messages::Packet;
use crate::server::messages::PacketStaticId;

pub struct ReceiptGatewayHandler;

#[async_trait]
impl GatewayHandler for ReceiptGatewayHandler {
    fn get_id(&self) -> i32 {
        <ContextRead as PacketStaticId>::get_id()
    }

    async fn handle(&self, user: &User, state: &mut RwLockWriteGuard<AppState>, message: &PacketMessage) {
        let request = ContextRead::decode_data(&message.data).expect("Failed to decode context read packet");
        let query = diesel::update(messagesTable)
            .filter(messageChannelId.eq(user.user_id).and(reception_status.ne(2)).and(user_id.eq(request.context_id)))
            .set(reception_status.eq(2))
            .returning(messageId);
        let returns = { query.load::<i64>(&mut state.database).expect("Failed to update reception status") };

        let target_tx = state.packet_queue.get(&request.context_id);
        if let Some(tx) = target_tx {
            println!("Sending messages read...");
            tx.send(Box::new(MessagesRead {
                reader_id: user.user_id,
                message_ids: returns
            })).await.expect("Failed to send messages read packet");
        }
    }
}