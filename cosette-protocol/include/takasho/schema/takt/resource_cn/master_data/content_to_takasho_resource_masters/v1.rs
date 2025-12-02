#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContentToTakashoResourceMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub content_to_takasho_resource_master_id: i64,
    #[prost(enumeration = "super::super::content_type_masters::v1::Type", tag = "11")]
    pub content_type: i32,
    #[prost(
        enumeration = "super::super::super::super::super::common_featureset::resource::player_inventory::v1::ItemType",
        tag = "12"
    )]
    pub takasho_resource_type: i32,
    #[prost(string, tag = "14")]
    pub takasho_name: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContentToTakashoResourceMasters {
    #[prost(message, repeated, tag = "1")]
    pub content_to_takasho_resource_masters: ::prost::alloc::vec::Vec<
        ContentToTakashoResourceMaster,
    >,
}
