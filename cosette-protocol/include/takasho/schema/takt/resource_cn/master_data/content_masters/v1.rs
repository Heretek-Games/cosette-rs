#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContentMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub content_master_id: i64,
    #[prost(int64, tag = "3")]
    pub content_type_master_id: i64,
    #[prost(string, tag = "90")]
    pub name: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContentMasters {
    #[prost(message, repeated, tag = "1")]
    pub content_masters: ::prost::alloc::vec::Vec<ContentMaster>,
}
