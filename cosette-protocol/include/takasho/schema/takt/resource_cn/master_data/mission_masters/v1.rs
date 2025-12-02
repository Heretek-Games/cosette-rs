#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MissionMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub mission_master_id: i64,
    #[prost(enumeration = "MissionCategory", tag = "3")]
    pub mission_category: i32,
    #[prost(int64, repeated, tag = "4")]
    pub previous_mission_ids: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, tag = "5")]
    pub require_player_level: i64,
    #[prost(int64, tag = "6")]
    pub require_chapter_master_id: i64,
    #[prost(int64, tag = "7")]
    pub mission_type_id: i64,
    #[prost(int64, tag = "8")]
    pub mission_type_value1: i64,
    #[prost(int64, tag = "9")]
    pub mission_type_value2: i64,
    #[prost(int64, tag = "10")]
    pub mission_type_value3: i64,
    #[prost(int64, tag = "11")]
    pub get_point: i64,
    #[prost(string, repeated, tag = "12")]
    pub rewards: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, tag = "21")]
    pub panel_rank: i64,
    #[prost(enumeration = "PanelMissionType", tag = "22")]
    pub panel_mission_type: i32,
    #[prost(int64, tag = "23")]
    pub badge_master_id: i64,
    #[prost(int64, tag = "24")]
    pub mission_sub_category_master_id: i64,
    #[prost(int64, tag = "25")]
    pub required_total_index: i64,
    #[prost(string, tag = "31")]
    pub start_time: ::prost::alloc::string::String,
    #[prost(string, tag = "32")]
    pub end_time: ::prost::alloc::string::String,
    #[prost(int64, repeated, tag = "33")]
    pub mission_type_value4: ::prost::alloc::vec::Vec<i64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MissionMasters {
    #[prost(message, repeated, tag = "1")]
    pub mission_masters: ::prost::alloc::vec::Vec<MissionMaster>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MissionCategory {
    InvalidMissionCategory = 0,
    Daily = 1,
    Weekly = 2,
    Achievement = 3,
    Panel = 4,
    Bond = 5,
    Newbie = 6,
    NewbieActivity = 7,
    Target = 8,
    Challenge = 9,
    ActivityDaily = 10,
    ActivityDaily2 = 11,
    MasterSection = 12,
}
impl MissionCategory {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MissionCategory::InvalidMissionCategory => "INVALID_MISSION_CATEGORY",
            MissionCategory::Daily => "DAILY",
            MissionCategory::Weekly => "WEEKLY",
            MissionCategory::Achievement => "ACHIEVEMENT",
            MissionCategory::Panel => "PANEL",
            MissionCategory::Bond => "BOND",
            MissionCategory::Newbie => "NEWBIE",
            MissionCategory::NewbieActivity => "NEWBIE_ACTIVITY",
            MissionCategory::Target => "TARGET",
            MissionCategory::Challenge => "CHALLENGE",
            MissionCategory::ActivityDaily => "ACTIVITY_DAILY",
            MissionCategory::ActivityDaily2 => "ACTIVITY_DAILY2",
            MissionCategory::MasterSection => "MASTER_SECTION",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INVALID_MISSION_CATEGORY" => Some(Self::InvalidMissionCategory),
            "DAILY" => Some(Self::Daily),
            "WEEKLY" => Some(Self::Weekly),
            "ACHIEVEMENT" => Some(Self::Achievement),
            "PANEL" => Some(Self::Panel),
            "BOND" => Some(Self::Bond),
            "NEWBIE" => Some(Self::Newbie),
            "NEWBIE_ACTIVITY" => Some(Self::NewbieActivity),
            "TARGET" => Some(Self::Target),
            "CHALLENGE" => Some(Self::Challenge),
            "ACTIVITY_DAILY" => Some(Self::ActivityDaily),
            "ACTIVITY_DAILY2" => Some(Self::ActivityDaily2),
            "MASTER_SECTION" => Some(Self::MasterSection),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PanelMissionType {
    InvalidPanelMissionType = 0,
    NotFinalPanel = 1,
    FinalPanel = 2,
}
impl PanelMissionType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PanelMissionType::InvalidPanelMissionType => "INVALID_PANEL_MISSION_TYPE",
            PanelMissionType::NotFinalPanel => "NOT_FINAL_PANEL",
            PanelMissionType::FinalPanel => "FINAL_PANEL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INVALID_PANEL_MISSION_TYPE" => Some(Self::InvalidPanelMissionType),
            "NOT_FINAL_PANEL" => Some(Self::NotFinalPanel),
            "FINAL_PANEL" => Some(Self::FinalPanel),
            _ => None,
        }
    }
}
