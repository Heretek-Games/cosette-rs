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
    #[prost(sint64, tag = "9")]
    pub total_lot_num: i64,
    #[prost(message, repeated, tag = "10")]
    pub consume_resource_sets: ::prost::alloc::vec::Vec<
        super::super::consume_resource_set::v1::ConsumeResourceSet,
    >,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LootBoxPickup {
    #[prost(string, tag = "1")]
    pub loot_box_pickup_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub pickup_contents: ::prost::alloc::vec::Vec<LootBoxContent>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LootBoxProductWithPurchasedCount {
    #[prost(message, optional, tag = "1")]
    pub loot_product: ::core::option::Option<LootBoxProduct>,
    #[prost(sint64, tag = "2")]
    pub purchased_count: i64,
    #[prost(bool, tag = "3")]
    pub is_limited: bool,
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
    #[prost(bytes = "vec", tag = "4")]
    pub value: ::prost::alloc::vec::Vec<u8>,
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
    #[prost(bytes = "vec", tag = "5")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "6")]
    pub resource_name: ::prost::alloc::string::String,
    #[prost(sint64, tag = "7")]
    pub resource_num: i64,
    #[prost(string, tag = "8")]
    pub search_label: ::prost::alloc::string::String,
    #[prost(bool, tag = "9")]
    pub is_receive_directly: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Limit {
    #[prost(enumeration = "LimitType", tag = "1")]
    pub limit_type: i32,
    #[prost(sint64, tag = "2")]
    pub limit_num: i64,
    #[prost(string, tag = "3")]
    pub date_line: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LootBoxProbability {
    #[prost(string, tag = "1")]
    pub loot_box_product_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub loot_box_content_set_probabilities: ::prost::alloc::vec::Vec<
        LootBoxContentSetProbability,
    >,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LootBoxContentSetProbability {
    #[prost(string, tag = "1")]
    pub loot_box_content_set_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub loot_box_label_probabilities: ::prost::alloc::vec::Vec<LootBoxLabelProbability>,
    #[prost(message, repeated, tag = "3")]
    pub loot_box_content_probabilities_in_content_set: ::prost::alloc::vec::Vec<
        LootBoxContentProbabilityInContentSet,
    >,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LootBoxLabelProbability {
    #[prost(string, tag = "1")]
    pub label: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub probability: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub loot_box_content_probabilities_in_label: ::prost::alloc::vec::Vec<
        LootBoxContentProbabilityInLabel,
    >,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LootBoxContentProbabilityInLabel {
    #[prost(string, tag = "1")]
    pub loot_box_content_id: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "3")]
    pub probability: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LootBoxContentProbabilityInContentSet {
    #[prost(string, tag = "1")]
    pub loot_box_content_id: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "3")]
    pub probability: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LootBoxContentSetPrizes {
    #[prost(string, tag = "1")]
    pub loot_box_content_set_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub loot_box_contents: ::prost::alloc::vec::Vec<LootBoxContent>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LimitType {
    None = 0,
    DateLine = 1,
    Total = 2,
}
impl LimitType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LimitType::None => "NONE",
            LimitType::DateLine => "DATE_LINE",
            LimitType::Total => "TOTAL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NONE" => Some(Self::None),
            "DATE_LINE" => Some(Self::DateLine),
            "TOTAL" => Some(Self::Total),
            _ => None,
        }
    }
}
