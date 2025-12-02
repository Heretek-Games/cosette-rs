#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BattleActivityBattleMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub battle_activity_battle_master_id: i64,
    #[prost(int64, tag = "30")]
    pub battle_activity_master_id: i64,
    #[prost(string, tag = "40")]
    pub battle_name: ::prost::alloc::string::String,
    #[prost(int64, tag = "50")]
    pub sort: i64,
    #[prost(int64, tag = "51")]
    pub stage_id: i64,
    #[prost(int64, tag = "52")]
    pub stage_type: i64,
    #[prost(int64, tag = "53")]
    pub quest_id: i64,
    #[prost(int64, tag = "60")]
    pub battle_id: i64,
    #[prost(int64, tag = "65")]
    pub battle_id_repeat: i64,
    #[prost(int64, tag = "70")]
    pub pre_piot_id: i64,
    #[prost(string, repeated, tag = "80")]
    pub require_condition: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "90")]
    pub rewards: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, tag = "100")]
    pub battle_type: i64,
    #[prost(string, repeated, tag = "110")]
    pub consume: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, repeated, tag = "120")]
    pub drop_id: ::prost::alloc::vec::Vec<i64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BattleActivityBattleMasters {
    #[prost(message, repeated, tag = "1")]
    pub battle_activity_battle_masters: ::prost::alloc::vec::Vec<
        BattleActivityBattleMaster,
    >,
}
