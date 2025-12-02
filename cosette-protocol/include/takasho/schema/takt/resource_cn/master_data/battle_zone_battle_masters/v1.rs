#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BattleZoneBattleMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub battle_zone_battle_master_id: i64,
    #[prost(int64, tag = "3")]
    pub battle_zone_master_id: i64,
    #[prost(int64, tag = "4")]
    pub battle_master_id: i64,
    #[prost(int64, repeated, tag = "5")]
    pub pre_id: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, tag = "6")]
    pub use_stamina: i64,
    #[prost(string, repeated, tag = "7")]
    pub first_clear_rewards: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bool, tag = "9")]
    pub is_boss: bool,
    #[prost(int64, tag = "10")]
    pub exp_player: i64,
    #[prost(int64, tag = "11")]
    pub exp_battle_character: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BattleZoneBattleMasters {
    #[prost(message, repeated, tag = "1")]
    pub battle_zone_battle_masters: ::prost::alloc::vec::Vec<BattleZoneBattleMaster>,
}
