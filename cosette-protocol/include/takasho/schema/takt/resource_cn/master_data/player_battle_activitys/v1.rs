#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerBattleActivity {
    #[prost(string, tag = "1")]
    pub player_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub battle_activity_master_id: i64,
    #[prost(int64, repeated, tag = "3")]
    pub piots: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, repeated, tag = "4")]
    pub battles: ::prost::alloc::vec::Vec<i64>,
    #[prost(string, tag = "40")]
    pub battles_star: ::prost::alloc::string::String,
    #[prost(int64, tag = "5")]
    pub hang_battle_id: i64,
    #[prost(int64, tag = "6")]
    pub hang_start_time: i64,
    #[prost(int64, tag = "7")]
    pub hang_rounds: i64,
    #[prost(int64, tag = "8")]
    pub hang_per_battle_time: i64,
    #[prost(string, tag = "9")]
    pub battle_min_time: ::prost::alloc::string::String,
    #[prost(int64, tag = "10")]
    pub battle_start_time: i64,
    #[prost(int64, tag = "11")]
    pub current_battle_id: i64,
}
