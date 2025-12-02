#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShopPopGiftMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub shop_pop_gift_master_id: i64,
    #[prost(string, repeated, tag = "3")]
    pub contents: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, tag = "4")]
    pub former_id: i64,
    #[prost(int64, tag = "5")]
    pub duration: i64,
    #[prost(enumeration = "GiftPopType", tag = "6")]
    pub pop_type: i32,
    #[prost(int64, tag = "7")]
    pub pop_times: i64,
    #[prost(enumeration = "TriggerType", tag = "8")]
    pub trigger_type: i32,
    #[prost(int64, tag = "9")]
    pub trigger_param1: i64,
    #[prost(int64, tag = "10")]
    pub trigger_param2: i64,
    #[prost(enumeration = "Region", tag = "11")]
    pub region: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShopPopGiftMasters {
    #[prost(message, repeated, tag = "1")]
    pub shop_pop_gift_masters: ::prost::alloc::vec::Vec<ShopPopGiftMaster>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum GiftPopType {
    InvalidGiftPopType = 0,
    Limit = 1,
    Daily = 2,
    Weekly = 3,
    Monthly = 4,
}
impl GiftPopType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            GiftPopType::InvalidGiftPopType => "Invalid_GiftPopType",
            GiftPopType::Limit => "Limit",
            GiftPopType::Daily => "Daily",
            GiftPopType::Weekly => "Weekly",
            GiftPopType::Monthly => "Monthly",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Invalid_GiftPopType" => Some(Self::InvalidGiftPopType),
            "Limit" => Some(Self::Limit),
            "Daily" => Some(Self::Daily),
            "Weekly" => Some(Self::Weekly),
            "Monthly" => Some(Self::Monthly),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TriggerType {
    InvalidTriggerType = 0,
    ChapterType = 1,
    LevelType = 2,
    CharacterType = 3,
    ContinuousLoginType = 4,
    CharacterBondType = 5,
    FirstGachaType = 6,
    BattleLoseType = 7,
    StaminaCostType = 8,
}
impl TriggerType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TriggerType::InvalidTriggerType => "Invalid_TriggerType",
            TriggerType::ChapterType => "ChapterType",
            TriggerType::LevelType => "LevelType",
            TriggerType::CharacterType => "CharacterType",
            TriggerType::ContinuousLoginType => "ContinuousLoginType",
            TriggerType::CharacterBondType => "CharacterBondType",
            TriggerType::FirstGachaType => "FirstGachaType",
            TriggerType::BattleLoseType => "BattleLoseType",
            TriggerType::StaminaCostType => "StaminaCostType",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Invalid_TriggerType" => Some(Self::InvalidTriggerType),
            "ChapterType" => Some(Self::ChapterType),
            "LevelType" => Some(Self::LevelType),
            "CharacterType" => Some(Self::CharacterType),
            "ContinuousLoginType" => Some(Self::ContinuousLoginType),
            "CharacterBondType" => Some(Self::CharacterBondType),
            "FirstGachaType" => Some(Self::FirstGachaType),
            "BattleLoseType" => Some(Self::BattleLoseType),
            "StaminaCostType" => Some(Self::StaminaCostType),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Region {
    Invalid = 0,
    Cn = 1,
    Jp = 2,
    Ww = 3,
}
impl Region {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Region::Invalid => "INVALID",
            Region::Cn => "CN",
            Region::Jp => "JP",
            Region::Ww => "WW",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INVALID" => Some(Self::Invalid),
            "CN" => Some(Self::Cn),
            "JP" => Some(Self::Jp),
            "WW" => Some(Self::Ww),
            _ => None,
        }
    }
}
