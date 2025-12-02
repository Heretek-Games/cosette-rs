#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DamageLimitationMaster {
    #[prost(string, tag = "99")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "1")]
    pub damage_limitation_master_id: i64,
    #[prost(int64, tag = "2")]
    pub character_id: i64,
    #[prost(int64, tag = "3")]
    pub level: i64,
    #[prost(int64, tag = "4")]
    pub damage_limit: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DamageLimitationMasters {
    #[prost(message, repeated, tag = "1")]
    pub damage_limitation_masters: ::prost::alloc::vec::Vec<DamageLimitationMaster>,
}
