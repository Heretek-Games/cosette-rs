#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BannedPlayer {
    #[prost(string, tag = "1")]
    pub player_id: ::prost::alloc::string::String,
    #[prost(sint64, tag = "2")]
    pub status: i64,
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    #[prost(sint64, tag = "4")]
    pub created_at: i64,
    #[prost(sint64, tag = "5")]
    pub updated_at: i64,
    #[prost(sint64, tag = "6")]
    pub expired_at: i64,
    #[prost(bool, tag = "7")]
    pub has_expired_at: bool,
}
