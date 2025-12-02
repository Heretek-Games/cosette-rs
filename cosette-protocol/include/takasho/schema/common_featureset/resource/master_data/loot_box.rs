#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LootBoxes {
    #[prost(message, repeated, tag = "1")]
    pub loot_box_products: ::prost::alloc::vec::Vec<LootBoxProduct>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LootBoxProduct {
    #[prost(string, tag = "1")]
    pub loot_box_product_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub virtual_currency_key: ::prost::alloc::string::String,
    #[prost(sint64, tag = "3")]
    pub virtual_currency_amount: i64,
    #[prost(string, tag = "4")]
    pub inventory_message: ::prost::alloc::string::String,
    #[prost(sint64, tag = "5")]
    pub opened_at: i64,
    #[prost(sint64, tag = "6")]
    pub closed_at: i64,
    #[prost(message, repeated, tag = "7")]
    pub content_sets: ::prost::alloc::vec::Vec<LootBoxContentSet>,
    #[prost(bool, tag = "8")]
    pub has_extra: bool,
    #[prost(message, optional, tag = "9")]
    pub extra: ::core::option::Option<Extra>,
    #[prost(bool, tag = "10")]
    pub has_limit: bool,
    #[prost(message, optional, tag = "11")]
    pub limit: ::core::option::Option<Limit>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LootBoxContentSet {
    #[prost(string, tag = "1")]
    pub loot_box_content_set_id: ::prost::alloc::string::String,
    #[prost(sint64, tag = "2")]
    pub lot_num: i64,
    #[prost(message, repeated, tag = "3")]
    pub contents: ::prost::alloc::vec::Vec<LootBoxContent>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LootBoxContent {
    #[prost(string, tag = "1")]
    pub loot_box_content_id: ::prost::alloc::string::String,
    #[prost(enumeration = "super::super::player_inventory::v1::ItemType", tag = "2")]
    pub item_type: i32,
    #[prost(string, tag = "3")]
    pub schema_key: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub value_path: ::prost::alloc::string::String,
    #[prost(sint64, tag = "5")]
    pub weight: i64,
    #[prost(string, tag = "6")]
    pub label: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub search_label: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Extra {
    #[prost(string, tag = "1")]
    pub inventory_message: ::prost::alloc::string::String,
    #[prost(enumeration = "super::super::player_inventory::v1::ItemType", tag = "2")]
    pub item_type: i32,
    #[prost(string, tag = "3")]
    pub schema_key: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "4")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "5")]
    pub value_path: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub resource_name: ::prost::alloc::string::String,
    #[prost(sint64, tag = "7")]
    pub resource_num: i64,
    #[prost(string, tag = "8")]
    pub search_label: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub transaction_payload: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Limit {
    #[prost(enumeration = "super::super::loot_box::v1::LimitType", tag = "1")]
    pub limit_type: i32,
    #[prost(sint64, tag = "2")]
    pub limit_num: i64,
    #[prost(string, tag = "3")]
    pub date_line: ::prost::alloc::string::String,
}
