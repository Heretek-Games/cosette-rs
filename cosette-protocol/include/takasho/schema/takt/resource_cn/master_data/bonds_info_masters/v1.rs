#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BondsInfoMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub bonds_info_master_id: i64,
    #[prost(int64, tag = "3")]
    pub battle_character_master_id: i64,
    #[prost(int64, repeated, tag = "4")]
    pub mission_master_id: ::prost::alloc::vec::Vec<i64>,
    #[prost(string, repeated, tag = "5")]
    pub reward_contents: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BondsInfoMasters {
    #[prost(message, repeated, tag = "1")]
    pub bonds_info_masters: ::prost::alloc::vec::Vec<BondsInfoMaster>,
}
