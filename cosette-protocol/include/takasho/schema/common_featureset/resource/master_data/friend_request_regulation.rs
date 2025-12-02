#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FriendRequestRegulation {
    #[prost(message, optional, tag = "1")]
    pub friend_request_regulation: ::core::option::Option<Regulation>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Regulation {
    #[prost(sint64, tag = "1")]
    pub rerequest_time: i64,
    #[prost(sint64, tag = "2")]
    pub max_send_friend_request: i64,
    #[prost(sint64, tag = "3")]
    pub max_receivable_friend_request: i64,
}
