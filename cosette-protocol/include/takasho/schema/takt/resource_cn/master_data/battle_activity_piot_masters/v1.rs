#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BattleActivityPiotMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub battle_activity_piot_master_id: i64,
    #[prost(int64, tag = "3")]
    pub battle_activity_master_id: i64,
    #[prost(string, tag = "4")]
    pub plot_name: ::prost::alloc::string::String,
    #[prost(int64, tag = "5")]
    pub sort: i64,
    #[prost(int64, tag = "7")]
    pub pre_battle_id: i64,
    #[prost(string, repeated, tag = "8")]
    pub require_condition: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "90")]
    pub rewards: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BattleActivityPiotMasters {
    #[prost(message, repeated, tag = "1")]
    pub battle_activity_piot_masters: ::prost::alloc::vec::Vec<BattleActivityPiotMaster>,
}
