#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UndergroundLevelMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub underground_level_master_id: i64,
    #[prost(string, tag = "11")]
    pub name: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UndergroundLevelMasters {
    #[prost(message, repeated, tag = "1")]
    pub underground_level_masters: ::prost::alloc::vec::Vec<UndergroundLevelMaster>,
}
