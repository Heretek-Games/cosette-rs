#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivateTalkMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub private_talk_master_id: i64,
    #[prost(int64, tag = "3")]
    pub keyword_id: i64,
    #[prost(int64, tag = "4")]
    pub character_master_id: i64,
    #[prost(int64, tag = "5")]
    pub battle_character_master_id: i64,
    #[prost(bool, tag = "6")]
    pub can_rand_drop: bool,
    #[prost(int64, tag = "7")]
    pub bond_level: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivateTalkMasters {
    #[prost(message, repeated, tag = "1")]
    pub private_talk_masters: ::prost::alloc::vec::Vec<PrivateTalkMaster>,
}
