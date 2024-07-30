use iris_macros::packet;
use prost::Message;

pub trait Packet {
    fn get_id(&self) -> i32;
    fn encode_data(&self) -> Vec<u8>;
    fn decode_data(buffer: &[u8]) -> Result<Self, prost::DecodeError> where Self: Sized;
}

// workaround
pub trait PacketStaticId {
    fn get_id() -> i32;
}

pub fn encode_packet_message(packet: Box<dyn Packet>) -> Vec<u8> {
    PacketMessage {
        id: packet.get_id(),
        data: packet.encode_data()
    }.encode_to_vec()
}

#[derive(Clone, PartialEq, Message)]
pub struct PacketMessage {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(bytes, tag = "2")]
    pub data: Vec<u8>
}

// SERVERBOUND

#[derive(Clone, PartialEq, Message)]
#[packet(id = 1)]
pub struct ChannelRead {
    #[prost(int64, tag = "1")]
    pub channel_id: i64
}

#[derive(Clone, PartialEq, Message)]
#[packet(id = 2)]
pub struct TypingRequest {
    #[prost(int64, tag = "1")]
    pub context_id: i64
}

// CLIENTBOUND

#[derive(Clone, PartialEq, Message)]
#[packet(id = 2)]
pub struct MessageCreated {
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(string, tag = "2")]
    pub content: String,
    #[prost(int64, tag = "3")]
    pub user_id: i64,
    #[prost(int64, tag = "4")]
    pub channel_id: i64,
    #[prost(int64, tag = "5", optional)]
    pub reply_to: Option<i64>
}

#[derive(Clone, PartialEq, Message)]
#[packet(id = 3)]
pub struct MessagesRead {
    #[prost(int64, tag = "1")]
    pub reader_id: i64,
    #[prost(int64, repeated, packed, tag = "2")]
    pub message_ids: Vec<i64>
}

#[derive(Clone, PartialEq, Message)]
#[packet(id = 4)]
pub struct ContactTyping {
    #[prost(int64, tag = "1")]
    pub contact_id: i64
}

#[derive(Clone, PartialEq, Message)]
#[packet(id = 5)]
pub struct MessageEdited {
    #[prost(int64, tag = "1")]
    pub message_id: i64,
    #[prost(int64, tag = "2")]
    pub channel_id: i64,
    #[prost(int64, tag = "3")]
    pub editor_id: i64,
    #[prost(string, tag = "4")]
    pub new_content: String
}

#[derive(Clone, PartialEq, Message)]
#[packet(id = 6)]
pub struct MessageDeleted {
    #[prost(int64, tag = "1")]
    pub message_id: i64,
    #[prost(int64, tag = "2")]
    pub channel_id: i64
}

#[derive(Clone, PartialEq, Message)]
#[packet(id = 7)]
pub struct ReactionAdded {
    #[prost(int64, tag = "1")]
    pub message_id: i64,
    #[prost(int64, tag = "2")]
    pub user_id: i64,
    #[prost(string, tag = "3")]
    pub emoji: String,
    #[prost(int32, tag = "4")]
    pub reaction_count: i32,
    #[prost(int32, tag = "5")]
    pub reaction_id: i32,
    #[prost(int64, tag = "6")]
    pub context_id: i64
}

#[derive(Clone, PartialEq, Message)]
#[packet(id = 8)]
pub struct ReactionRemoved {
    #[prost(int64, tag = "1")]
    pub message_id: i64,
    #[prost(int64, tag = "2")]
    pub user_id: i64,
    #[prost(string, tag = "3")]
    pub emoji: String,
    #[prost(int32, tag = "4")]
    pub reaction_count: i32,
    #[prost(int32, tag = "5")]
    pub reaction_id: i32,
    #[prost(int64, tag = "6")]
    pub context_id: i64
}