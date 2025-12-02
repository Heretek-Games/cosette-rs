#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerKeyValue {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(sint64, tag = "2")]
    pub value: i64,
    #[prost(sint64, tag = "3")]
    pub expired_at: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerKeyValueIncrementInfo {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(sint64, tag = "2")]
    pub delta: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerKeyValueUpdateInfo {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(sint64, tag = "2")]
    pub amount: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OperationType {
    Unknown = 0,
    Increase = 1,
    Decrease = 2,
}
impl OperationType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OperationType::Unknown => "UNKNOWN",
            OperationType::Increase => "INCREASE",
            OperationType::Decrease => "DECREASE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN" => Some(Self::Unknown),
            "INCREASE" => Some(Self::Increase),
            "DECREASE" => Some(Self::Decrease),
            _ => None,
        }
    }
}
