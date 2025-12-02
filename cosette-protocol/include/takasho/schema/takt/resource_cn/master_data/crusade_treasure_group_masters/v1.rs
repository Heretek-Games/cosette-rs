#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CrusadeTreasureGroupMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub crusade_treasure_group_master_id: i64,
    #[prost(int64, repeated, tag = "3")]
    pub treasure_master_ids: ::prost::alloc::vec::Vec<i64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CrusadeTreasureGroupMasters {
    #[prost(message, repeated, tag = "1")]
    pub crusade_treasure_group_masters: ::prost::alloc::vec::Vec<
        CrusadeTreasureGroupMaster,
    >,
}
