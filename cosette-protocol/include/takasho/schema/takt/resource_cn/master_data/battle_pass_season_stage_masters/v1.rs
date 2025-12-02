#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BattlePassSeasonStageMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub battle_pass_season_stage_master_id: i64,
    #[prost(int64, tag = "3")]
    pub stage_level: i64,
    #[prost(int64, tag = "4")]
    pub battle_pass_season_id: i64,
    #[prost(string, repeated, tag = "5")]
    pub purchase_rewards: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, tag = "6")]
    pub shop_product_master_id: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BattlePassSeasonStageMasters {
    #[prost(message, repeated, tag = "1")]
    pub battle_pass_season_stage_masters: ::prost::alloc::vec::Vec<
        BattlePassSeasonStageMaster,
    >,
}
