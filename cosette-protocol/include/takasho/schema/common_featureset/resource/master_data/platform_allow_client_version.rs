#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlatformAllowClientVersions {
    #[prost(message, repeated, tag = "1")]
    pub platform_allow_client_versions: ::prost::alloc::vec::Vec<
        PlatformAllowClientVersion,
    >,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlatformAllowClientVersion {
    #[prost(enumeration = "super::super::system::v1::PlatformType", tag = "1")]
    pub platform_type: i32,
    #[prost(string, tag = "2")]
    pub client_version: ::prost::alloc::string::String,
}
