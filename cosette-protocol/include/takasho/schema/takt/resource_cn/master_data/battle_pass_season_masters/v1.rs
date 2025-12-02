#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BattlePassSeasonMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub battle_pass_season_master_id: i64,
    #[prost(int64, tag = "3")]
    pub stage_count: i64,
    #[prost(int64, tag = "4")]
    pub max_level: i64,
    #[prost(string, tag = "5")]
    pub start_time: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub end_time: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "7")]
    pub purchase_cost: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, tag = "8")]
    pub exp_level: i64,
    #[prost(int64, tag = "9")]
    pub week_exp_limit: i64,
    #[prost(
        enumeration = "super::super::super::super::super::score::resource_cn::system::v1::Region",
        tag = "10"
    )]
    pub region: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BattlePassSeasonMasters {
    #[prost(message, repeated, tag = "1")]
    pub battle_pass_season_masters: ::prost::alloc::vec::Vec<BattlePassSeasonMaster>,
}
