#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Achievement {
    #[prost(string, tag = "1")]
    pub achievement_id: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[prost(sint64, tag = "4")]
    pub opened_at: i64,
    #[prost(sint64, tag = "5")]
    pub closed_at: i64,
    #[prost(enumeration = "ResetType", tag = "6")]
    pub reset_type: i32,
    #[prost(string, tag = "7")]
    pub date_line: ::prost::alloc::string::String,
    #[prost(sint64, tag = "8")]
    pub day_of_week: i64,
    #[prost(message, repeated, tag = "11")]
    pub prizes: ::prost::alloc::vec::Vec<AchievementPrize>,
    #[prost(string, tag = "12")]
    pub category: ::prost::alloc::string::String,
    #[prost(bool, tag = "13")]
    pub unlock: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AchievementPrize {
    #[prost(string, tag = "1")]
    pub achievement_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub achievement_prize_id: ::prost::alloc::string::String,
    #[prost(enumeration = "super::super::player_inventory::v1::ItemType", tag = "4")]
    pub item_type: i32,
    #[prost(string, tag = "5")]
    pub schema_key: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "6")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "7")]
    pub search_label: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub system_resource_name: ::prost::alloc::string::String,
    #[prost(sint64, tag = "9")]
    pub system_resource_num: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Criterion {
    #[prost(string, tag = "1")]
    pub category: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ResetType {
    None = 0,
    DateLine = 1,
    WeekLine = 2,
}
impl ResetType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ResetType::None => "NONE",
            ResetType::DateLine => "DATE_LINE",
            ResetType::WeekLine => "WEEK_LINE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NONE" => Some(Self::None),
            "DATE_LINE" => Some(Self::DateLine),
            "WEEK_LINE" => Some(Self::WeekLine),
            _ => None,
        }
    }
}
