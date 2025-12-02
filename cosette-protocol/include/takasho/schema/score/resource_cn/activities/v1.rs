#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpActivities {
    #[prost(int64, tag = "1")]
    pub op_activity_id: i64,
    #[prost(string, repeated, tag = "2")]
    pub affcodes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, repeated, tag = "3")]
    pub realms: ::prost::alloc::vec::Vec<i64>,
    #[prost(enumeration = "OpActivityCode", tag = "4")]
    pub code: i32,
    #[prost(string, tag = "5")]
    pub name: ::prost::alloc::string::String,
    #[prost(int64, tag = "6")]
    pub start_time: i64,
    #[prost(int64, tag = "7")]
    pub end_time: i64,
    #[prost(string, tag = "8")]
    pub payload: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "13")]
    pub payload2: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bool, tag = "9")]
    pub multi: bool,
    #[prost(string, tag = "10")]
    pub banner_path: ::prost::alloc::string::String,
    #[prost(bool, tag = "11")]
    pub is_open: bool,
    #[prost(int64, tag = "12")]
    pub sort: i64,
    #[prost(bool, tag = "14")]
    pub is_pre_publish: bool,
    #[prost(string, repeated, tag = "15")]
    pub players: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(enumeration = "OpActivityBanner", tag = "16")]
    pub with_banner: i32,
    #[prost(int64, tag = "17")]
    pub client_config_id: i64,
    #[prost(sint64, tag = "91")]
    pub created_at: i64,
    #[prost(sint64, tag = "92")]
    pub updated_at: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OpActivityCode {
    OpActivityInvalid = 0,
    OpActivityRebate = 1,
    OpActivityCrusade = 2,
    OpActivityWeb = 3,
    OpActivityQuestionnaire = 4,
    OpActivityGacha = 5,
    OpActivityBattle = 6,
    OpActivityCommonCrusade = 7,
    OpActivityMasterSection = 8,
    OpActivityDailyQuest = 9,
}
impl OpActivityCode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OpActivityCode::OpActivityInvalid => "OP_ACTIVITY_INVALID",
            OpActivityCode::OpActivityRebate => "OP_ACTIVITY_REBATE",
            OpActivityCode::OpActivityCrusade => "OP_ACTIVITY_CRUSADE",
            OpActivityCode::OpActivityWeb => "OP_ACTIVITY_WEB",
            OpActivityCode::OpActivityQuestionnaire => "OP_ACTIVITY_QUESTIONNAIRE",
            OpActivityCode::OpActivityGacha => "OP_ACTIVITY_GACHA",
            OpActivityCode::OpActivityBattle => "OP_ACTIVITY_BATTLE",
            OpActivityCode::OpActivityCommonCrusade => "OP_ACTIVITY_COMMON_CRUSADE",
            OpActivityCode::OpActivityMasterSection => "OP_ACTIVITY_MASTER_SECTION",
            OpActivityCode::OpActivityDailyQuest => "OP_ACTIVITY_DAILY_QUEST",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OP_ACTIVITY_INVALID" => Some(Self::OpActivityInvalid),
            "OP_ACTIVITY_REBATE" => Some(Self::OpActivityRebate),
            "OP_ACTIVITY_CRUSADE" => Some(Self::OpActivityCrusade),
            "OP_ACTIVITY_WEB" => Some(Self::OpActivityWeb),
            "OP_ACTIVITY_QUESTIONNAIRE" => Some(Self::OpActivityQuestionnaire),
            "OP_ACTIVITY_GACHA" => Some(Self::OpActivityGacha),
            "OP_ACTIVITY_BATTLE" => Some(Self::OpActivityBattle),
            "OP_ACTIVITY_COMMON_CRUSADE" => Some(Self::OpActivityCommonCrusade),
            "OP_ACTIVITY_MASTER_SECTION" => Some(Self::OpActivityMasterSection),
            "OP_ACTIVITY_DAILY_QUEST" => Some(Self::OpActivityDailyQuest),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OpActivityBanner {
    Both = 0,
    OnlyBanner = 1,
    NoBanner = 2,
}
impl OpActivityBanner {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OpActivityBanner::Both => "BOTH",
            OpActivityBanner::OnlyBanner => "ONLY_BANNER",
            OpActivityBanner::NoBanner => "NO_BANNER",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "BOTH" => Some(Self::Both),
            "ONLY_BANNER" => Some(Self::OnlyBanner),
            "NO_BANNER" => Some(Self::NoBanner),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerOpActivityRecords {
    #[prost(int64, tag = "1")]
    pub op_activity_id: i64,
    #[prost(string, tag = "2")]
    pub player_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub realm: ::prost::alloc::string::String,
    #[prost(int64, tag = "4")]
    pub update_time: i64,
    #[prost(string, tag = "5")]
    pub pay_load: ::prost::alloc::string::String,
    #[prost(sint64, tag = "91")]
    pub created_at: i64,
    #[prost(sint64, tag = "92")]
    pub updated_at: i64,
}
