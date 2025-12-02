#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CrusadeEnvironmentMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub crusade_environment_master_id: i64,
    #[prost(int64, tag = "3")]
    pub initial_treasure_id: i64,
    #[prost(int64, tag = "4")]
    pub crusade_type_master_id: i64,
    #[prost(string, tag = "7")]
    pub time: ::prost::alloc::string::String,
    #[prost(int64, tag = "8")]
    pub crusade_adv_master_id: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CrusadeEnvironmentMasters {
    #[prost(message, repeated, tag = "1")]
    pub crusade_environment_masters: ::prost::alloc::vec::Vec<CrusadeEnvironmentMaster>,
}
