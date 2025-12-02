#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GameProduct {
    #[prost(string, tag = "1")]
    pub game_product_id: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub schema_key: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "7")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "8")]
    pub inventory_message: ::prost::alloc::string::String,
    #[prost(sint64, tag = "9")]
    pub opened_at: i64,
    #[prost(sint64, tag = "10")]
    pub closed_at: i64,
    #[prost(sint64, tag = "11")]
    pub receivable_sec: i64,
    #[prost(string, tag = "12")]
    pub search_label: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "13")]
    pub consume_resource_sets: ::prost::alloc::vec::Vec<
        super::super::consume_resource_set::v1::ConsumeResourceSet,
    >,
}
