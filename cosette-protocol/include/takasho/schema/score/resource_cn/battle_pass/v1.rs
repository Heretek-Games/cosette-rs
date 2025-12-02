#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerBattlePass {
    #[prost(string, tag = "1")]
    pub player_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub battle_pass_season_id: i64,
    #[prost(int64, tag = "3")]
    pub stage_level: i64,
    #[prost(int64, tag = "4")]
    pub exp_unlimit: i64,
    #[prost(int64, tag = "5")]
    pub exp_limit: i64,
    #[prost(int64, tag = "6")]
    pub exp_weekly: i64,
    #[prost(int64, tag = "7")]
    pub purchase_level_times: i64,
    #[prost(sint64, tag = "8")]
    pub last_refresh_time: i64,
    #[prost(sint64, tag = "91")]
    pub created_at: i64,
    #[prost(sint64, tag = "92")]
    pub updated_at: i64,
}
