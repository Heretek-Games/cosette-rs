#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Compensations {
    #[prost(message, repeated, tag = "1")]
    pub compensations: ::prost::alloc::vec::Vec<Compensation>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Compensation {
    #[prost(enumeration = "super::super::player_inventory::v1::ItemType", tag = "1")]
    pub item_type: i32,
    #[prost(string, tag = "2")]
    pub schema_key: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub value_path: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub message: ::prost::alloc::string::String,
    #[prost(sint64, tag = "5")]
    pub receivable_sec: i64,
    #[prost(string, tag = "6")]
    pub search_label: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub system_resource_name: ::prost::alloc::string::String,
    #[prost(sint64, tag = "8")]
    pub system_resource_num: i64,
    #[prost(string, tag = "9")]
    pub transaction_payload: ::prost::alloc::string::String,
}
