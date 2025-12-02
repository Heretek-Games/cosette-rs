#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CrusadeMapShowMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub crusade_map_show_master_id: i64,
    #[prost(string, tag = "3")]
    pub start: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub end: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CrusadeMapShowMasters {
    #[prost(message, repeated, tag = "1")]
    pub crusade_map_show_masters: ::prost::alloc::vec::Vec<CrusadeMapShowMaster>,
}
