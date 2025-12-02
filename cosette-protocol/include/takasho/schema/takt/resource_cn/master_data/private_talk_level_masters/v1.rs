#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivateTalkLevelMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub private_talk_level_master_id: i64,
    #[prost(int64, tag = "3")]
    pub keyword_level: i64,
    #[prost(int64, tag = "4")]
    pub bond_level: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivateTalkLevelMasters {
    #[prost(message, repeated, tag = "1")]
    pub private_talk_level_masters: ::prost::alloc::vec::Vec<PrivateTalkLevelMaster>,
}
