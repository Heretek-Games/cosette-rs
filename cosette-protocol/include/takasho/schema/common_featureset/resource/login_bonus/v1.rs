#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoginBonus {
    #[prost(string, tag = "1")]
    pub login_bonus_id: ::prost::alloc::string::String,
    #[prost(sint64, tag = "4")]
    pub opened_at: i64,
    #[prost(sint64, tag = "5")]
    pub closed_at: i64,
    #[prost(enumeration = "ResetType", tag = "6")]
    pub reset_type: i32,
    #[prost(string, tag = "7")]
    pub date_line: ::prost::alloc::string::String,
    #[prost(sint64, tag = "8")]
    pub time_lapse: i64,
    #[prost(sint64, tag = "9")]
    pub last_prize_count: i64,
    #[prost(sint64, tag = "10")]
    pub last_counted_up_at: i64,
    #[prost(message, repeated, tag = "11")]
    pub prizes: ::prost::alloc::vec::Vec<LoginBonusPrize>,
    #[prost(bool, tag = "12")]
    pub r#loop: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoginBonusPrize {
    #[prost(string, tag = "1")]
    pub login_bonus_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub login_bonus_prize_id: ::prost::alloc::string::String,
    #[prost(sint64, tag = "3")]
    pub prize_count: i64,
    #[prost(enumeration = "super::super::player_inventory::v1::ItemType", tag = "4")]
    pub item_type: i32,
    #[prost(string, tag = "5")]
    pub schema_key: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "6")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "7")]
    pub search_label: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub resource_name: ::prost::alloc::string::String,
    #[prost(sint64, tag = "9")]
    pub resource_num: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ResetType {
    DateLine = 0,
    TimeLapse = 1,
}
impl ResetType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ResetType::DateLine => "DATE_LINE",
            ResetType::TimeLapse => "TIME_LAPSE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DATE_LINE" => Some(Self::DateLine),
            "TIME_LAPSE" => Some(Self::TimeLapse),
            _ => None,
        }
    }
}
