#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CrusadeLevelMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub crusade_level_master_id: i64,
    #[prost(int64, tag = "3")]
    pub crusade_environment_master_id: i64,
    #[prost(int64, repeated, tag = "4")]
    pub crusade_map_show_master_ids: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, tag = "5")]
    pub recommend_level: i64,
    #[prost(int64, repeated, tag = "6")]
    pub supports: ::prost::alloc::vec::Vec<i64>,
    #[prost(string, repeated, tag = "7")]
    pub resource_limitation: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "8")]
    pub unlock_condition: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CrusadeLevelMasters {
    #[prost(message, repeated, tag = "1")]
    pub crusade_level_masters: ::prost::alloc::vec::Vec<CrusadeLevelMaster>,
}
