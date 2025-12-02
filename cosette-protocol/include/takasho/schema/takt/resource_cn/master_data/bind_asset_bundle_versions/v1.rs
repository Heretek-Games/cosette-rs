#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BindAssetBundleVersion {
    #[prost(string, tag = "30")]
    pub account_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "40")]
    pub platform_type: i64,
    #[prost(string, tag = "50")]
    pub asset_bundle_version: ::prost::alloc::string::String,
    #[prost(sint64, tag = "91")]
    pub created_at: i64,
    #[prost(sint64, tag = "92")]
    pub updated_at: i64,
}
