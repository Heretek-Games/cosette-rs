#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlatformDenyClientVersionSettings {
    #[prost(message, repeated, tag = "1")]
    pub platform_deny_client_version_settings: ::prost::alloc::vec::Vec<
        PlatformDenyClientVersionSetting,
    >,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlatformDenyClientVersionSetting {
    #[prost(enumeration = "super::super::system::v1::PlatformType", tag = "1")]
    pub platform_type: i32,
    #[prost(string, tag = "2")]
    pub payload: ::prost::alloc::string::String,
}
