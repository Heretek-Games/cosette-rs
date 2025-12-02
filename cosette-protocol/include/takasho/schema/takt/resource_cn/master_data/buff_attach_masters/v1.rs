#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuffAttachMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub buff_attach_master_id: i64,
    #[prost(int64, tag = "3")]
    pub buff_id: i64,
    #[prost(int64, tag = "4")]
    pub target_type: i64,
    #[prost(int64, tag = "5")]
    pub turn: i64,
    #[prost(int64, tag = "6")]
    pub stock: i64,
    #[prost(int64, tag = "7")]
    pub success_percent: i64,
    #[prost(int64, repeated, tag = "8")]
    pub precondition_buffs: ::prost::alloc::vec::Vec<i64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuffAttachMasters {
    #[prost(message, repeated, tag = "1")]
    pub buff_attach_masters: ::prost::alloc::vec::Vec<BuffAttachMaster>,
}
