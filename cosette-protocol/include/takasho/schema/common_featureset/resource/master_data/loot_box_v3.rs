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
    pub inventory_message: ::prost::alloc::string::String,
    #[prost(sint64, tag = "3")]
    pub opened_at: i64,
    #[prost(sint64, tag = "4")]
    pub closed_at: i64,
    #[prost(bool, tag = "5")]
    pub has_extra: bool,
    #[prost(message, repeated, tag = "6")]
    pub extras: ::prost::alloc::vec::Vec<Extra>,
    #[prost(bool, tag = "7")]
    pub has_limit: bool,
    #[prost(message, optional, tag = "8")]
    pub limit: ::core::option::Option<Limit>,
    #[prost(message, repeated, tag = "9")]
    pub consume_resource_sets: ::prost::alloc::vec::Vec<
        super::consume_resource_set::ConsumeResourceSet,
    >,
    #[prost(message, repeated, tag = "10")]
    pub content_sets: ::prost::alloc::vec::Vec<LootBoxContentSet>,
    #[prost(message, repeated, tag = "11")]
    pub pickups: ::prost::alloc::vec::Vec<Pickup>,
    #[prost(string, tag = "12")]
    pub transaction_payload: ::prost::alloc::string::String,
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
    pub labels: ::prost::alloc::vec::Vec<LootBoxLabel>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LootBoxLabel {
    #[prost(string, tag = "1")]
    pub loot_box_label: ::prost::alloc::string::String,
    #[prost(sint64, tag = "2")]
    pub weight: i64,
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
    pub search_label: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Extra {
    #[prost(string, tag = "1")]
    pub loot_box_extra_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub inventory_message: ::prost::alloc::string::String,
    #[prost(enumeration = "super::super::player_inventory::v1::ItemType", tag = "3")]
    pub item_type: i32,
    #[prost(string, tag = "4")]
    pub schema_key: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub value_path: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub system_resource_name: ::prost::alloc::string::String,
    #[prost(sint64, tag = "7")]
    pub system_resource_num: i64,
    #[prost(string, tag = "8")]
    pub search_label: ::prost::alloc::string::String,
    #[prost(bool, tag = "9")]
    pub is_receive_directly: bool,
    #[prost(string, tag = "10")]
    pub transaction_payload: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Limit {
    #[prost(enumeration = "super::super::loot_box::v3::LimitType", tag = "1")]
    pub limit_type: i32,
    #[prost(sint64, tag = "2")]
    pub limit_num: i64,
    #[prost(string, tag = "3")]
    pub date_line: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pickup {
    #[prost(string, tag = "1")]
    pub loot_box_pickup_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub pickup_contents: ::prost::alloc::vec::Vec<PickupContent>,
    #[prost(message, repeated, tag = "3")]
    pub pickup_extras: ::prost::alloc::vec::Vec<PickupExtra>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PickupContent {
    #[prost(string, tag = "1")]
    pub loot_box_pickup_content_id: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PickupExtra {
    #[prost(string, tag = "1")]
    pub loot_box_pickup_extra_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub inventory_message: ::prost::alloc::string::String,
    #[prost(enumeration = "super::super::player_inventory::v1::ItemType", tag = "3")]
    pub item_type: i32,
    #[prost(string, tag = "4")]
    pub schema_key: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub value_path: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub system_resource_name: ::prost::alloc::string::String,
    #[prost(sint64, tag = "7")]
    pub system_resource_num: i64,
    #[prost(string, tag = "8")]
    pub search_label: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub transaction_payload: ::prost::alloc::string::String,
}
