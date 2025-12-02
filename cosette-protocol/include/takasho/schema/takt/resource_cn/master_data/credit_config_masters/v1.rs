#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreditConfigMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub credit_config_master_id: i64,
    #[prost(int64, tag = "3")]
    pub threshold: i64,
    #[prost(int64, tag = "4")]
    pub days: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreditConfigMasters {
    #[prost(message, repeated, tag = "1")]
    pub credit_config_masters: ::prost::alloc::vec::Vec<CreditConfigMaster>,
}
