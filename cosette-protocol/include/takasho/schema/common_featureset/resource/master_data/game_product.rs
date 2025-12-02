#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GameProducts {
    #[prost(message, repeated, tag = "1")]
    pub game_products: ::prost::alloc::vec::Vec<GameProduct>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GameProduct {
    #[prost(string, tag = "1")]
    pub game_product_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub schema_key: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub value_path: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub inventory_message: ::prost::alloc::string::String,
    #[prost(sint64, tag = "5")]
    pub opened_at: i64,
    #[prost(sint64, tag = "6")]
    pub closed_at: i64,
    #[prost(sint64, tag = "7")]
    pub receivable_sec: i64,
    #[prost(message, repeated, tag = "8")]
    pub consume_resource_sets: ::prost::alloc::vec::Vec<
        super::consume_resource_set::ConsumeResourceSet,
    >,
    #[prost(string, tag = "9")]
    pub search_label: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub transaction_payload: ::prost::alloc::string::String,
    #[prost(enumeration = "super::super::player_inventory::v1::ItemType", tag = "50")]
    pub item_type: i32,
    #[prost(string, tag = "51")]
    pub system_resource_name: ::prost::alloc::string::String,
    #[prost(sint64, tag = "52")]
    pub system_resource_num: i64,
}
