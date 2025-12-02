#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CrusadeMapMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub crusade_map_master_id: i64,
    #[prost(int64, tag = "3")]
    pub crusade_map_show_master_id: i64,
    #[prost(int64, repeated, tag = "4")]
    pub next: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, repeated, tag = "5")]
    pub crusade_event_group_master_ids: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, tag = "6")]
    pub node_type: i64,
    #[prost(string, repeated, tag = "7")]
    pub unlock_condition: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CrusadeMapMasters {
    #[prost(message, repeated, tag = "1")]
    pub crusade_map_masters: ::prost::alloc::vec::Vec<CrusadeMapMaster>,
}
