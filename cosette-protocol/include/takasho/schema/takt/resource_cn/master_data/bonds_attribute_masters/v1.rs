#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BondsAttributeMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub bonds_attribute_master_id: i64,
    #[prost(int64, tag = "3")]
    pub battle_character_master_id: i64,
    #[prost(int64, tag = "4")]
    pub plate_id: i64,
    #[prost(int64, tag = "5")]
    pub require_bonds_level: i64,
    #[prost(string, repeated, tag = "6")]
    pub unlock_reward_contents: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BondsAttributeMasters {
    #[prost(message, repeated, tag = "1")]
    pub bonds_attribute_masters: ::prost::alloc::vec::Vec<BondsAttributeMaster>,
}
