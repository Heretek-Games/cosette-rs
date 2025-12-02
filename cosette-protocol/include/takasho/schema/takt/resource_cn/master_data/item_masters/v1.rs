#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ItemMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub item_master_id: i64,
    #[prost(enumeration = "super::super::content_type_masters::v1::Type", tag = "11")]
    pub content_type: i32,
    #[prost(int64, tag = "12")]
    pub sort_id: i64,
    #[prost(enumeration = "Rarity", tag = "13")]
    pub rarity: i32,
    #[prost(int64, tag = "14")]
    pub drop_master_id: i64,
    #[prost(int64, tag = "16")]
    pub sell_price: i64,
    #[prost(int64, tag = "17")]
    pub exp: i64,
    #[prost(string, tag = "90")]
    pub name: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ItemMasters {
    #[prost(message, repeated, tag = "1")]
    pub item_masters: ::prost::alloc::vec::Vec<ItemMaster>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Rarity {
    InvalidRarity = 0,
}
impl Rarity {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Rarity::InvalidRarity => "INVALID_RARITY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INVALID_RARITY" => Some(Self::InvalidRarity),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DisplayType {
    InvalidDisplayType = 0,
}
impl DisplayType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DisplayType::InvalidDisplayType => "INVALID_DISPLAY_TYPE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INVALID_DISPLAY_TYPE" => Some(Self::InvalidDisplayType),
            _ => None,
        }
    }
}
