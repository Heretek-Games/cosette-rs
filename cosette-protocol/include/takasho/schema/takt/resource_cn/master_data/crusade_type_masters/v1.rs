#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CrusadeTypeMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub crusade_type_master_id: i64,
    #[prost(string, tag = "3")]
    pub start: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub end: ::prost::alloc::string::String,
    #[prost(int64, tag = "5")]
    pub extra_rank: i64,
    #[prost(int64, repeated, tag = "6")]
    pub extra_buff: ::prost::alloc::vec::Vec<i64>,
    #[prost(enumeration = "CrusadeType", tag = "7")]
    pub r#type: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CrusadeTypeMasters {
    #[prost(message, repeated, tag = "1")]
    pub crusade_type_masters: ::prost::alloc::vec::Vec<CrusadeTypeMaster>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CrusadeType {
    Normal = 0,
    Activity = 1,
}
impl CrusadeType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CrusadeType::Normal => "NORMAL",
            CrusadeType::Activity => "ACTIVITY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NORMAL" => Some(Self::Normal),
            "ACTIVITY" => Some(Self::Activity),
            _ => None,
        }
    }
}
