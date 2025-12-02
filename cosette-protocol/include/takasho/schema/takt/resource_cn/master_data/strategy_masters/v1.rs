#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StrategyMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub strategy_master_id: i64,
    #[prost(string, tag = "3")]
    pub strategy_type: ::prost::alloc::string::String,
    #[prost(int64, tag = "4")]
    pub priority: i64,
    #[prost(int64, tag = "5")]
    pub effect_value1: i64,
    #[prost(int64, tag = "6")]
    pub effect_value2: i64,
    #[prost(int64, tag = "7")]
    pub effect_value3: i64,
    #[prost(int64, tag = "8")]
    pub effect_value4: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StrategyMasters {
    #[prost(message, repeated, tag = "1")]
    pub strategy_masters: ::prost::alloc::vec::Vec<StrategyMaster>,
}
