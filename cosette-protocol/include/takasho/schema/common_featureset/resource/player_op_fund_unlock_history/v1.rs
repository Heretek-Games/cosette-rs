#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerOpFundUnlockHistory {
    #[prost(string, tag = "1")]
    pub player_op_event_history_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub player_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub op_fund_unlock_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub op_event_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "5")]
    pub unlocked_at: i64,
}
