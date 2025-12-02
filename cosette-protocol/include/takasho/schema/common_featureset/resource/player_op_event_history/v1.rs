#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerOpEventHistory {
    #[prost(string, tag = "1")]
    pub player_op_event_history_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub transaction_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub op_event_id: ::prost::alloc::string::String,
    #[prost(enumeration = "super::super::op_event::v1::OpEventType", tag = "4")]
    pub op_event_type: i32,
    #[prost(string, tag = "5")]
    pub player_id: ::prost::alloc::string::String,
}
