#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConsumeResourceSet {
    #[prost(string, tag = "1")]
    pub consume_resource_set_id: ::prost::alloc::string::String,
    #[prost(enumeration = "ResourceType", tag = "2")]
    pub resource_type: i32,
    #[prost(int64, tag = "3")]
    pub amount: i64,
    #[prost(message, repeated, tag = "4")]
    pub consume_resources: ::prost::alloc::vec::Vec<ConsumeResource>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConsumeResource {
    #[prost(string, tag = "1")]
    pub consume_resource_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub resource_key: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    pub priority: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ResourceType {
    VirtualCurrency = 0,
    PlayerKeyValue = 1,
    NoConsume = 2,
}
impl ResourceType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ResourceType::VirtualCurrency => "VIRTUAL_CURRENCY",
            ResourceType::PlayerKeyValue => "PLAYER_KEY_VALUE",
            ResourceType::NoConsume => "NO_CONSUME",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "VIRTUAL_CURRENCY" => Some(Self::VirtualCurrency),
            "PLAYER_KEY_VALUE" => Some(Self::PlayerKeyValue),
            "NO_CONSUME" => Some(Self::NoConsume),
            _ => None,
        }
    }
}
