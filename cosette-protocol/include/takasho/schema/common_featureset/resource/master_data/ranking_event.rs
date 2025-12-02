#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RankingEvents {
    #[prost(message, repeated, tag = "1")]
    pub ranking_events: ::prost::alloc::vec::Vec<RankingEvent>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RankingEvent {
    #[prost(string, tag = "1")]
    pub ranking_event_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub schema_key: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub value_path: ::prost::alloc::string::String,
    #[prost(sint64, tag = "4")]
    pub opened_at: i64,
    #[prost(sint64, tag = "5")]
    pub closed_at: i64,
}
