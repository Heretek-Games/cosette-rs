#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LanguageMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub language_master_id: i64,
    #[prost(string, tag = "20")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag = "30")]
    pub jp: ::prost::alloc::string::String,
    #[prost(string, tag = "40")]
    pub en: ::prost::alloc::string::String,
    #[prost(string, tag = "50")]
    pub cn: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LanguageMasters {
    #[prost(message, repeated, tag = "1")]
    pub language_masters: ::prost::alloc::vec::Vec<LanguageMaster>,
}
