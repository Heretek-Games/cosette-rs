#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CrusadeEventGroupMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub crusade_event_group_master_id: i64,
    #[prost(int64, repeated, tag = "3")]
    pub crusade_event_master_ids: ::prost::alloc::vec::Vec<i64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CrusadeEventGroupMasters {
    #[prost(message, repeated, tag = "1")]
    pub crusade_event_group_masters: ::prost::alloc::vec::Vec<CrusadeEventGroupMaster>,
}
