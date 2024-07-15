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
pub struct ContextRead {
    #[prost(int64, tag = "1")]
    pub context_id: i64
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
pub struct TextMessageResponse {
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(string, tag = "2")]
    pub content: String,
    #[prost(int64, tag = "3")]
    pub user_id: i64,
    #[prost(int64, tag = "4")]
    pub context: i64,
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