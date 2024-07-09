use prost::Message;

#[derive(Clone, PartialEq, Message)]
pub struct TextMessage {
    #[prost(string, tag = "1")]
    pub content: String
}