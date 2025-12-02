#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BattleCharacterBondMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub battle_character_bond_master_id: i64,
    #[prost(string, tag = "11")]
    pub name: ::prost::alloc::string::String,
    #[prost(int64, repeated, tag = "12")]
    pub favorite: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, repeated, tag = "13")]
    pub like: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, repeated, tag = "14")]
    pub normal: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, repeated, tag = "15")]
    pub hate: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, repeated, tag = "21")]
    pub favorite_teabreak: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, repeated, tag = "22")]
    pub like_teabreak: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, repeated, tag = "23")]
    pub normal_teabreak: ::prost::alloc::vec::Vec<i64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BattleCharacterBondMasters {
    #[prost(message, repeated, tag = "1")]
    pub battle_character_bond_masters: ::prost::alloc::vec::Vec<
        BattleCharacterBondMaster,
    >,
}
