#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RealmBinaryVersionBinding {
    #[prost(string, tag = "1")]
    pub realm_binary_version_binding_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub binary_version_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub affcode_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, repeated, tag = "4")]
    pub initial_realm_ids: ::prost::alloc::vec::Vec<i64>,
    #[prost(sint64, tag = "91")]
    pub created_at: i64,
    #[prost(sint64, tag = "92")]
    pub updated_at: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BinaryVersion {
    #[prost(string, tag = "1")]
    pub binary_version_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub binary_version: ::prost::alloc::string::String,
    #[prost(
        enumeration = "super::super::super::super::common_featureset::resource::system::v1::PlatformType",
        tag = "3"
    )]
    pub platform: i32,
    #[prost(sint64, tag = "91")]
    pub created_at: i64,
    #[prost(sint64, tag = "92")]
    pub updated_at: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BinaryVersionMasterAssetBinding {
    #[prost(string, tag = "1")]
    pub binary_version_master_asset_binding_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub binary_version_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub affcode_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "4")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub asset_bundle_version_id: ::prost::alloc::string::String,
    #[prost(sint64, tag = "91")]
    pub created_at: i64,
    #[prost(sint64, tag = "92")]
    pub updated_at: i64,
}
