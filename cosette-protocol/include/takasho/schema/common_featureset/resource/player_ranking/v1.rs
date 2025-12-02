#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerRankInfo {
    #[prost(string, tag = "1")]
    pub ranking_key: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub player_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    pub score: i64,
    #[prost(int64, tag = "4")]
    pub rank: i64,
    #[prost(string, tag = "5")]
    pub nickname: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RankingMaster {
    #[prost(string, tag = "1")]
    pub ranking_key: ::prost::alloc::string::String,
    #[prost(sint64, tag = "2")]
    pub opened_at: i64,
    #[prost(sint64, tag = "3")]
    pub closed_at: i64,
    #[prost(bytes = "vec", tag = "4")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
