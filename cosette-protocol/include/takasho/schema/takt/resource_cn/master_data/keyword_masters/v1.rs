#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub keyword_master_id: i64,
    #[prost(string, tag = "3")]
    pub keyword: ::prost::alloc::string::String,
    #[prost(int64, tag = "4")]
    pub level: i64,
    #[prost(int64, repeated, tag = "5")]
    pub battle_character_master_ids: ::prost::alloc::vec::Vec<i64>,
    #[prost(enumeration = "UseType", tag = "6")]
    pub r#type: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordMasters {
    #[prost(message, repeated, tag = "1")]
    pub keyword_masters: ::prost::alloc::vec::Vec<KeywordMaster>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum UseType {
    DefaultUseType = 0,
    MustOwnedBeforeUseType = 1,
}
impl UseType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            UseType::DefaultUseType => "Default_UseType",
            UseType::MustOwnedBeforeUseType => "MustOwnedBefore_UseType",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Default_UseType" => Some(Self::DefaultUseType),
            "MustOwnedBefore_UseType" => Some(Self::MustOwnedBeforeUseType),
            _ => None,
        }
    }
}
