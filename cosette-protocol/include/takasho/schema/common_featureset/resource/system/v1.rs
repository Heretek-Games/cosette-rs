#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceInfo {
    #[prost(enumeration = "PlatformType", tag = "1")]
    pub platform: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerStatus {
    #[prost(sint64, tag = "1")]
    pub status: i64,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(sint64, tag = "3")]
    pub expired_at: i64,
    #[prost(bool, tag = "4")]
    pub has_expired_at: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BanStatus {
    #[prost(sint64, tag = "1")]
    pub status: i64,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(sint64, tag = "3")]
    pub expired_at: i64,
    #[prost(bool, tag = "4")]
    pub has_expired_at: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PlatformType {
    Unknown = 0,
    Google = 1,
    Apple = 2,
    Editor = 3,
}
impl PlatformType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PlatformType::Unknown => "UNKNOWN",
            PlatformType::Google => "GOOGLE",
            PlatformType::Apple => "APPLE",
            PlatformType::Editor => "EDITOR",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN" => Some(Self::Unknown),
            "GOOGLE" => Some(Self::Google),
            "APPLE" => Some(Self::Apple),
            "EDITOR" => Some(Self::Editor),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BaasType {
    Npf = 0,
    Lcx = 1,
}
impl BaasType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            BaasType::Npf => "NPF",
            BaasType::Lcx => "LCX",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NPF" => Some(Self::Npf),
            "LCX" => Some(Self::Lcx),
            _ => None,
        }
    }
}
