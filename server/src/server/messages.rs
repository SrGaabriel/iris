use iris_macros::packet;
use prost::Message;

pub trait Packet {
    fn get_id(&self) -> i32;
    fn encode_data(&self) -> Vec<u8>;
    fn decode_data(buffer: &[u8]) -> Result<Self, serde_json::Error> where Self: Sized;
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