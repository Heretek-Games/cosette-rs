#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReceivedFriendRequest {
    #[prost(message, optional, tag = "1")]
    pub from_player: ::core::option::Option<Player>,
    #[prost(sint64, tag = "2")]
    pub sent_at: i64,
    #[prost(string, tag = "3")]
    pub schema_key: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "4")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MyFriendRequest {
    #[prost(message, optional, tag = "1")]
    pub to_player: ::core::option::Option<Player>,
    #[prost(sint64, tag = "2")]
    pub sent_at: i64,
    #[prost(string, tag = "3")]
    pub schema_key: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "4")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Player {
    #[prost(string, tag = "1")]
    pub player_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub player_storage_entries: ::prost::alloc::vec::Vec<
        super::super::player_storage::v2::Entry,
    >,
    #[prost(sint64, tag = "3")]
    pub last_logged_in_at: i64,
    #[prost(string, tag = "4")]
    pub nickname: ::prost::alloc::string::String,
    #[prost(sint64, tag = "5")]
    pub followed_at: i64,
}
