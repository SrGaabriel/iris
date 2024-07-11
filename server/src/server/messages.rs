use prost::Message;

#[derive(Clone, PartialEq, Message)]
pub struct TextMessageRequest {
    #[prost(string, tag = "1")]
    pub content: String
}

#[derive(Clone, PartialEq, Message)]
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