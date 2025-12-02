#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GachaMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub gacha_master_id: i64,
    #[prost(string, tag = "90")]
    pub name: ::prost::alloc::string::String,
    #[prost(enumeration = "Kind", tag = "3")]
    pub kind: i32,
    #[prost(enumeration = "Type", tag = "4")]
    pub r#type: i32,
    #[prost(int64, tag = "5")]
    pub up_group_id: i64,
    #[prost(bool, tag = "6")]
    pub has_daily_limit: bool,
    #[prost(string, repeated, tag = "7")]
    pub daily_limit_cost: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "8")]
    pub draw_one_cost: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "81")]
    pub draw_one_cost2: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, tag = "9")]
    pub ssr_guaranteed_gacha_odd_master_id: i64,
    #[prost(int64, tag = "10")]
    pub ssr_guaranteed_count: i64,
    #[prost(int64, tag = "11")]
    pub sr_guaranteed_count: i64,
    #[prost(int64, tag = "12")]
    pub up_weight_start_count: i64,
    #[prost(int64, tag = "13")]
    pub up_weight_percentage: i64,
    #[prost(int64, tag = "14")]
    pub maximum_count_per_day: i64,
    #[prost(bool, tag = "15")]
    pub skip_history: bool,
    #[prost(
        enumeration = "super::super::super::super::super::score::resource_cn::common::v1::LimitType",
        tag = "16"
    )]
    pub limit_type: i32,
    #[prost(int64, tag = "17")]
    pub limit_count: i64,
    #[prost(int64, tag = "18")]
    pub limit_cooldown_hours: i64,
    #[prost(int64, tag = "19")]
    pub up_weight_upper_limit: i64,
    #[prost(int64, repeated, tag = "20")]
    pub up_icon_ids: ::prost::alloc::vec::Vec<i64>,
    #[prost(bool, tag = "21")]
    pub reset_security: bool,
    #[prost(int64, tag = "22")]
    pub common_guaranteed_count: i64,
    #[prost(int64, tag = "23")]
    pub common_guaranteed_gacha_odd_master_id: i64,
    #[prost(bool, tag = "24")]
    pub allow_gem_convert_stone: bool,
    #[prost(string, repeated, tag = "25")]
    pub unlock_condition: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GachaMasters {
    #[prost(message, repeated, tag = "1")]
    pub gacha_masters: ::prost::alloc::vec::Vec<GachaMaster>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Kind {
    InvalidKind = 0,
    Character = 1,
    Equipment = 2,
    Explore = 3,
}
impl Kind {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Kind::InvalidKind => "INVALID_KIND",
            Kind::Character => "KIND_CHARACTER",
            Kind::Equipment => "KIND_EQUIPMENT",
            Kind::Explore => "KIND_EXPLORE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INVALID_KIND" => Some(Self::InvalidKind),
            "KIND_CHARACTER" => Some(Self::Character),
            "KIND_EQUIPMENT" => Some(Self::Equipment),
            "KIND_EXPLORE" => Some(Self::Explore),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Type {
    InvalidType = 0,
    Persistent = 1,
    Activity = 2,
    Limited = 3,
}
impl Type {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Type::InvalidType => "INVALID_TYPE",
            Type::Persistent => "TYPE_PERSISTENT",
            Type::Activity => "TYPE_ACTIVITY",
            Type::Limited => "TYPE_LIMITED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INVALID_TYPE" => Some(Self::InvalidType),
            "TYPE_PERSISTENT" => Some(Self::Persistent),
            "TYPE_ACTIVITY" => Some(Self::Activity),
            "TYPE_LIMITED" => Some(Self::Limited),
            _ => None,
        }
    }
}
