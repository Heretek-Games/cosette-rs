#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerCredit {
    #[prost(string, tag = "1")]
    pub player_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub purchase_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    pub credit_point: i64,
    #[prost(int64, tag = "4")]
    pub last_update_at: i64,
    #[prost(sint64, tag = "91")]
    pub created_at: i64,
    #[prost(sint64, tag = "92")]
    pub updated_at: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerCreditPointResponse {
    #[prost(int64, tag = "1")]
    pub credit_points: i64,
    #[prost(int64, tag = "2")]
    pub base_time: i64,
    #[prost(int64, tag = "3")]
    pub countdown_time: i64,
    #[prost(int64, tag = "4")]
    pub reach_level_master_id: i64,
}
