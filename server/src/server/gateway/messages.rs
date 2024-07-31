use iris_macros::packet;
use serde::{Deserialize, Serialize};
use crate::server::rest::{MessageObject, StandardUser};
// SERVERBOUND

#[packet(id = 1)]
pub struct ChannelRead {
    pub channel_id: i64
}

#[packet(id = 2)]
pub struct TypingRequest {
    pub channel_id: i64
}

// CLIENTBOUND

#[packet(id = 2)]
pub struct MessageCreated {
    pub message: MessageObject
}

#[packet(id = 3)]
pub struct MessagesRead {
    pub reader_id: i64,
    pub message_ids: Vec<i64>
}

#[packet(id = 4)]
pub struct ChannelTyping {
    pub user: StandardUser,
    pub channel_id: i64
}

#[packet(id = 5)]
pub struct MessageEdited {
    pub message_id: i64,
    pub channel_id: i64,
    pub editor_id: i64,
    pub new_content: String
}

#[packet(id = 6)]
pub struct MessageDeleted {
    pub message_id: i64,
    pub channel_id: i64
}

#[packet(id = 7)]
pub struct ReactionAdded {
    pub message_id: i64,
    pub user_id: i64,
    pub emoji: String,
    pub reaction_count: i32,
    pub reaction_id: i32,
    pub channel_id: i64
}

#[packet(id = 8)]
pub struct ReactionRemoved {
    pub message_id: i64,
    pub user_id: i64,
    pub emoji: String,
    pub reaction_count: i32,
    pub reaction_id: i32,
    pub channel_id: i64
}