#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BaasProduct {
    #[prost(string, tag = "1")]
    pub baas_product_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub inventory_message: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub extras: ::prost::alloc::vec::Vec<BaasProductExtra>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BaasProductExtra {
    #[prost(string, tag = "1")]
    pub baas_product_extra_id: ::prost::alloc::string::String,
    #[prost(enumeration = "super::super::player_inventory::v1::ItemType", tag = "2")]
    pub item_type: i32,
    #[prost(string, tag = "3")]
    pub schema_key: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "4")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "5")]
    pub system_resource_name: ::prost::alloc::string::String,
    #[prost(sint64, tag = "6")]
    pub system_resource_num: i64,
    #[prost(string, tag = "7")]
    pub search_label: ::prost::alloc::string::String,
}
