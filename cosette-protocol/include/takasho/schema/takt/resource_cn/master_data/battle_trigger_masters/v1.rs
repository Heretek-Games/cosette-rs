#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BattleTriggerMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub battle_trigger_master_id: i64,
    #[prost(string, tag = "3")]
    pub trigger_type: ::prost::alloc::string::String,
    #[prost(int64, repeated, tag = "4")]
    pub effect_values: ::prost::alloc::vec::Vec<i64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BattleTriggerMasters {
    #[prost(message, repeated, tag = "1")]
    pub battle_trigger_masters: ::prost::alloc::vec::Vec<BattleTriggerMaster>,
}
