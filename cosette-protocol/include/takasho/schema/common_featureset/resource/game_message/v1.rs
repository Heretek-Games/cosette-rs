#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GameMessage {
    #[prost(string, tag = "1")]
    pub message_id: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub is_received: bool,
    #[prost(string, tag = "3")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub message: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "5")]
    pub extra: ::prost::alloc::vec::Vec<u8>,
    #[prost(sint64, tag = "6")]
    pub created_at: i64,
}
