#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BoxLootBoxProduct {
    #[prost(string, tag = "1")]
    pub box_loot_box_product_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub inventory_message: ::prost::alloc::string::String,
    #[prost(sint64, tag = "3")]
    pub opened_at: i64,
    #[prost(sint64, tag = "4")]
    pub closed_at: i64,
    #[prost(sint64, tag = "5")]
    pub item_amount: i64,
    #[prost(sint64, tag = "6")]
    pub box_reset_count_limit: i64,
    #[prost(bool, tag = "7")]
    pub has_extra: bool,
    #[prost(message, repeated, tag = "8")]
    pub extras: ::prost::alloc::vec::Vec<Extra>,
    #[prost(message, repeated, tag = "9")]
    pub consume_resource_sets: ::prost::alloc::vec::Vec<
        super::super::consume_resource_set::v1::ConsumeResourceSet,
    >,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BoxLootBoxContent {
    #[prost(string, tag = "1")]
    pub box_loot_box_product_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub box_loot_box_content_id: ::prost::alloc::string::String,
    #[prost(enumeration = "super::super::player_inventory::v1::ItemType", tag = "3")]
    pub item_type: i32,
    #[prost(string, tag = "4")]
    pub schema_key: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "5")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[prost(sint64, tag = "6")]
    pub amount: i64,
    #[prost(string, tag = "7")]
    pub search_label: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Extra {
    #[prost(string, tag = "1")]
    pub box_loot_box_extra_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub inventory_message: ::prost::alloc::string::String,
    #[prost(enumeration = "super::super::player_inventory::v1::ItemType", tag = "3")]
    pub item_type: i32,
    #[prost(string, tag = "4")]
    pub schema_key: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "5")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "6")]
    pub resource_name: ::prost::alloc::string::String,
    #[prost(sint64, tag = "7")]
    pub resource_num: i64,
    #[prost(string, tag = "8")]
    pub search_label: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BoxLootBoxContentWithPlayerRemainContent {
    #[prost(string, tag = "1")]
    pub box_loot_box_product_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub box_loot_box_content_id: ::prost::alloc::string::String,
    #[prost(enumeration = "super::super::player_inventory::v1::ItemType", tag = "3")]
    pub item_type: i32,
    #[prost(string, tag = "4")]
    pub schema_key: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "5")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[prost(sint64, tag = "6")]
    pub amount: i64,
    #[prost(sint64, tag = "7")]
    pub player_remain_content_amount: i64,
}
