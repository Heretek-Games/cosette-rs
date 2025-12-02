#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerDailyQuestSweep {
    #[prost(string, tag = "1")]
    pub player_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub daily_quest_master_id: i64,
    #[prost(enumeration = "DailyQuestSweepStatus", tag = "3")]
    pub status: i32,
    #[prost(sint64, tag = "4")]
    pub start_at: i64,
    #[prost(sint64, tag = "5")]
    pub end_at: i64,
    #[prost(int64, tag = "6")]
    pub times: i64,
    #[prost(int64, tag = "7")]
    pub seed: i64,
    #[prost(sint64, tag = "91")]
    pub created_at: i64,
    #[prost(sint64, tag = "92")]
    pub updated_at: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DailyQuestSweepStatus {
    Spare = 0,
    Ongoing = 1,
    Finished = 2,
}
impl DailyQuestSweepStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DailyQuestSweepStatus::Spare => "SPARE",
            DailyQuestSweepStatus::Ongoing => "ONGOING",
            DailyQuestSweepStatus::Finished => "FINISHED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SPARE" => Some(Self::Spare),
            "ONGOING" => Some(Self::Ongoing),
            "FINISHED" => Some(Self::Finished),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerDailyQuest {
    #[prost(string, tag = "1")]
    pub player_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub daily_quest_master_id: i64,
    #[prost(int64, tag = "4")]
    pub normal_count: i64,
    #[prost(int64, tag = "5")]
    pub last_normal_timestamp: i64,
    #[prost(int64, tag = "6")]
    pub skip_count: i64,
    #[prost(int64, tag = "7")]
    pub last_skip_timestamp: i64,
    #[prost(bool, tag = "8")]
    pub already_three_star: bool,
    #[prost(int64, tag = "9")]
    pub least_battle_time: i64,
    #[prost(int64, repeated, tag = "10")]
    pub stars: ::prost::alloc::vec::Vec<i64>,
    #[prost(message, repeated, tag = "100")]
    pub afk_characters: ::prost::alloc::vec::Vec<LeastBattleTimeCharacter>,
    #[prost(string, repeated, tag = "101")]
    pub db_afk_characters: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(sint64, tag = "91")]
    pub created_at: i64,
    #[prost(sint64, tag = "92")]
    pub updated_at: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeastBattleTimeCharacter {
    #[prost(int64, tag = "1")]
    pub battle_character_id: i64,
    #[prost(int64, tag = "2")]
    pub rank: i64,
    #[prost(int64, tag = "3")]
    pub level: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DailyQuestBattleType {
    InvalidDailyQuestBattleType = 0,
    Normal = 1,
    Skip = 2,
}
impl DailyQuestBattleType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DailyQuestBattleType::InvalidDailyQuestBattleType => {
                "INVALID_DAILY_QUEST_BATTLE_TYPE"
            }
            DailyQuestBattleType::Normal => "NORMAL",
            DailyQuestBattleType::Skip => "SKIP",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INVALID_DAILY_QUEST_BATTLE_TYPE" => Some(Self::InvalidDailyQuestBattleType),
            "NORMAL" => Some(Self::Normal),
            "SKIP" => Some(Self::Skip),
            _ => None,
        }
    }
}
