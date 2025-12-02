#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StepUpLootBoxes {
    #[prost(message, repeated, tag = "1")]
    pub step_up_loot_box_products: ::prost::alloc::vec::Vec<StepUpLootBoxProduct>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StepUpLootBoxProduct {
    #[prost(string, tag = "1")]
    pub step_up_loot_box_product_id: ::prost::alloc::string::String,
    #[prost(sint64, tag = "2")]
    pub opened_at: i64,
    #[prost(sint64, tag = "3")]
    pub closed_at: i64,
    #[prost(bool, tag = "4")]
    pub is_loop: bool,
    #[prost(message, repeated, tag = "5")]
    pub steps: ::prost::alloc::vec::Vec<Step>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Step {
    #[prost(sint64, tag = "1")]
    pub step_up_loot_box_step_num: i64,
    #[prost(string, tag = "4")]
    pub inventory_message: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "5")]
    pub content_sets: ::prost::alloc::vec::Vec<StepUpLootBoxContentSet>,
    #[prost(bool, tag = "6")]
    pub has_extra: bool,
    #[prost(message, repeated, tag = "7")]
    pub extras: ::prost::alloc::vec::Vec<Extra>,
    #[prost(sint64, tag = "8")]
    pub repeat_num: i64,
    #[prost(message, repeated, tag = "9")]
    pub consume_resource_sets: ::prost::alloc::vec::Vec<
        super::consume_resource_set::ConsumeResourceSet,
    >,
    #[prost(string, tag = "10")]
    pub transaction_payload: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StepUpLootBoxContentSet {
    #[prost(string, tag = "1")]
    pub step_up_loot_box_content_set_id: ::prost::alloc::string::String,
    #[prost(sint64, tag = "2")]
    pub lot_num: i64,
    #[prost(message, repeated, tag = "3")]
    pub contents: ::prost::alloc::vec::Vec<StepUpLootBoxContent>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StepUpLootBoxContent {
    #[prost(string, tag = "1")]
    pub step_up_loot_box_content_id: ::prost::alloc::string::String,
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
    pub step_up_loot_box_extra_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub inventory_message: ::prost::alloc::string::String,
    #[prost(enumeration = "super::super::player_inventory::v1::ItemType", tag = "3")]
    pub item_type: i32,
    #[prost(string, tag = "4")]
    pub schema_key: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "5")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "6")]
    pub value_path: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub system_resource_name: ::prost::alloc::string::String,
    #[prost(sint64, tag = "8")]
    pub system_resource_num: i64,
    #[prost(string, tag = "9")]
    pub search_label: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub transaction_payload: ::prost::alloc::string::String,
}
