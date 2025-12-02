#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BattleZoneChestMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub battle_zone_chest_master_id: i64,
    #[prost(int64, tag = "3")]
    pub battle_zone_master_id: i64,
    #[prost(string, repeated, tag = "4")]
    pub chest_rewards: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, repeated, tag = "5")]
    pub open_condition: ::prost::alloc::vec::Vec<i64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BattleZoneChestMasters {
    #[prost(message, repeated, tag = "1")]
    pub battle_zone_chest_masters: ::prost::alloc::vec::Vec<BattleZoneChestMaster>,
}
