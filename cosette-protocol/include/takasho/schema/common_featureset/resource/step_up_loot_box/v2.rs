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
    #[prost(sint64, tag = "5")]
    pub loop_times: i64,
    #[prost(message, repeated, tag = "6")]
    pub step_up_loot_box_steps: ::prost::alloc::vec::Vec<StepUpLootBoxStep>,
    #[prost(sint64, tag = "7")]
    pub next_step_num: i64,
    #[prost(sint64, tag = "8")]
    pub next_step_repeat_times: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StepUpLootBoxStep {
    #[prost(sint64, tag = "1")]
    pub step_up_loot_box_step_num: i64,
    #[prost(string, tag = "2")]
    pub inventory_message: ::prost::alloc::string::String,
    #[prost(bool, tag = "3")]
    pub has_extra: bool,
    #[prost(message, repeated, tag = "4")]
    pub extras: ::prost::alloc::vec::Vec<Extra>,
    #[prost(sint64, tag = "5")]
    pub repeat_times: i64,
    #[prost(message, repeated, tag = "6")]
    pub step_up_loot_box_content_sets: ::prost::alloc::vec::Vec<StepUpLootBoxContentSet>,
    #[prost(message, repeated, tag = "7")]
    pub consume_resource_sets: ::prost::alloc::vec::Vec<
        super::super::consume_resource_set::v1::ConsumeResourceSet,
    >,
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
    pub labels: ::prost::alloc::vec::Vec<StepUpLootBoxLabel>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StepUpLootBoxLabel {
    #[prost(string, tag = "1")]
    pub step_up_loot_box_label: ::prost::alloc::string::String,
    #[prost(sint64, tag = "2")]
    pub weight: i64,
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
    #[prost(bytes = "vec", tag = "4")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[prost(sint64, tag = "5")]
    pub weight: i64,
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
    pub resource_name: ::prost::alloc::string::String,
    #[prost(sint64, tag = "7")]
    pub resource_num: i64,
    #[prost(string, tag = "8")]
    pub search_label: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StepUpLootBoxProbability {
    #[prost(string, tag = "1")]
    pub step_up_loot_box_product_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub step_up_loot_box_step_probabilities: ::prost::alloc::vec::Vec<
        StepUpLootBoxStepProbability,
    >,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StepUpLootBoxStepProbability {
    #[prost(sint64, tag = "1")]
    pub step_num: i64,
    #[prost(message, repeated, tag = "2")]
    pub step_up_loot_box_content_set_probabilities: ::prost::alloc::vec::Vec<
        StepUpLootBoxContentSetProbability,
    >,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StepUpLootBoxContentSetProbability {
    #[prost(string, tag = "1")]
    pub step_up_loot_box_content_set_id: ::prost::alloc::string::String,
    #[prost(sint64, tag = "2")]
    pub lot_num: i64,
    #[prost(message, repeated, tag = "3")]
    pub step_up_loot_box_label_probabilities: ::prost::alloc::vec::Vec<
        StepUpLootBoxLabelProbability,
    >,
    #[prost(message, repeated, tag = "4")]
    pub step_up_loot_box_content_probabilities_in_content_set: ::prost::alloc::vec::Vec<
        StepUpLootBoxContentProbabilityInContentSet,
    >,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StepUpLootBoxLabelProbability {
    #[prost(string, tag = "1")]
    pub label: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub probability: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub step_up_loot_box_content_probabilities_in_label: ::prost::alloc::vec::Vec<
        StepUpLootBoxContentProbabilityInLabel,
    >,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StepUpLootBoxContentProbabilityInLabel {
    #[prost(string, tag = "1")]
    pub step_up_loot_box_content_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub schema_key: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "3")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "4")]
    pub probability: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StepUpLootBoxContentProbabilityInContentSet {
    #[prost(string, tag = "1")]
    pub step_up_loot_box_content_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub schema_key: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "3")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "4")]
    pub probability: ::prost::alloc::string::String,
}
