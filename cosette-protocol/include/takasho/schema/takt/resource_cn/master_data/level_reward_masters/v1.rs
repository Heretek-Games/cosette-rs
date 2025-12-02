#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LevelRewardMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub level_reward_master_id: i64,
    #[prost(int64, tag = "3")]
    pub require_player_level: i64,
    #[prost(string, repeated, tag = "4")]
    pub rewards: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LevelRewardMasters {
    #[prost(message, repeated, tag = "1")]
    pub level_reward_masters: ::prost::alloc::vec::Vec<LevelRewardMaster>,
}
