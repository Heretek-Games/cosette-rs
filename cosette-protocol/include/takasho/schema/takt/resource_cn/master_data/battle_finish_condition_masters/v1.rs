#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BattleFinishConditionMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub battle_finish_condition_master_id: i64,
    #[prost(int64, tag = "3")]
    pub finish_type: i64,
    #[prost(int64, tag = "4")]
    pub enum_num: i64,
    #[prost(int64, tag = "5")]
    pub character_master_id: i64,
    #[prost(int64, tag = "6")]
    pub param_value1: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BattleFinishConditionMasters {
    #[prost(message, repeated, tag = "1")]
    pub battle_finish_condition_masters: ::prost::alloc::vec::Vec<
        BattleFinishConditionMaster,
    >,
}
