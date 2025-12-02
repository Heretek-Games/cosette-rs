#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AffcodeMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub affcode_master_id: i64,
    #[prost(string, tag = "3")]
    pub affcode: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub affcode_name: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AffcodeMasters {
    #[prost(message, repeated, tag = "1")]
    pub affcode_masters: ::prost::alloc::vec::Vec<AffcodeMaster>,
}
