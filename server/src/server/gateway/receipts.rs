use async_trait::async_trait;
use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl};
use tokio::sync::RwLockWriteGuard;

use crate::AppState;
use crate::schema::messages::messages::{channel_id as messageChannelId, channel_id, message_id as messageId, reception_status, user_id};
use crate::schema::messages::messages::dsl::messages as messagesTable;
use crate::schema::users::User;
use crate::server::gateway::GatewayHandler;
use crate::server::messages::{ChannelRead, MessagesRead, PacketMessage};
use crate::server::messages::Packet;
use crate::server::messages::PacketStaticId;

pub struct ReceiptGatewayHandler;

#[async_trait]
impl GatewayHandler for ReceiptGatewayHandler {
    fn get_id(&self) -> i32 {
        <ChannelRead as PacketStaticId>::get_id()
    }

    async fn handle(&self, user: &User, state: &mut RwLockWriteGuard<AppState>, message: &PacketMessage) {
        let request = ChannelRead::decode_data(&message.data).expect("Failed to decode context read packet");
        let query = diesel::update(messagesTable)
            .filter(channel_id.eq(request.channel_id))
            .set(reception_status.eq(2))
            .returning(messageId);
        let returns = { query.load::<i64>(&mut state.database).expect("Failed to update reception status") };

        println!("The receipts system is currently disabled.");
        // let target_tx = state.packet_queue.get(&request.channel_id);
        // if let Some(tx) = target_tx {
        //     println!("Sending messages read...");
        //     tx.send(Box::new(MessagesRead {
        //         reader_id: user.user_id,
        //         message_ids: returns
        //     })).await.expect("Failed to send messages read packet");
        // }
    }
}