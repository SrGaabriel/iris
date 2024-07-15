use std::ops::Deref;
use tokio::sync::RwLockWriteGuard;
use async_trait::async_trait;
use crate::{AppState, SharedState};
use crate::entity::user::User;
use crate::server::messages::{Packet, PacketMessage, PacketStaticId};

pub mod receipts;
pub mod typing;

#[async_trait]
pub trait GatewayHandler {
    fn get_id(&self) -> i32;

    fn accepts(&self, id: i32) -> bool {
        self.get_id() == id
    }

    async fn handle(&self, user: &User, state: &mut RwLockWriteGuard<AppState>, message: &PacketMessage);
}

pub struct Gateway {
    handlers: Vec<Box<dyn GatewayHandler + Send + Sync>>
}

impl Gateway {
    pub fn new() -> Gateway {
        Gateway {
            handlers: Vec::new()
        }
    }

    pub fn register_handler(&mut self, handler: Box<dyn GatewayHandler + Send + Sync>) {
        self.handlers.push(handler);
    }

    pub async fn handle_packet(&self, user: &User, state: &mut RwLockWriteGuard<'_, AppState>, packet_message: &PacketMessage) {
        for handler in self.handlers.iter() {
            if handler.accepts(packet_message.id) {
                handler.handle(user, state, packet_message).await;
            }
        }
    }
}

unsafe impl Send for Gateway {}
unsafe impl Sync for Gateway {}