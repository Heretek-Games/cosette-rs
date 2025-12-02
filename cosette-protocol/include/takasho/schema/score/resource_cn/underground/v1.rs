#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerUndergroundChapter {
    #[prost(string, tag = "1")]
    pub player_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub underground_chapter_master_id: i64,
    #[prost(enumeration = "ChapterStatus", tag = "3")]
    pub status: i32,
    #[prost(sint64, tag = "91")]
    pub created_at: i64,
    #[prost(sint64, tag = "92")]
    pub updated_at: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ChapterStatus {
    InvalidChapterStatus = 0,
    ChapterUnlocked = 1,
    ChapterStarted = 2,
}
impl ChapterStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ChapterStatus::InvalidChapterStatus => "INVALID_CHAPTER_STATUS",
            ChapterStatus::ChapterUnlocked => "CHAPTER_UNLOCKED",
            ChapterStatus::ChapterStarted => "CHAPTER_STARTED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INVALID_CHAPTER_STATUS" => Some(Self::InvalidChapterStatus),
            "CHAPTER_UNLOCKED" => Some(Self::ChapterUnlocked),
            "CHAPTER_STARTED" => Some(Self::ChapterStarted),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerUndergroundEvent {
    #[prost(string, tag = "1")]
    pub player_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub underground_event_master_id: i64,
    #[prost(enumeration = "EventStatus", tag = "3")]
    pub status: i32,
    #[prost(int64, tag = "81")]
    pub finished_count: i64,
    #[prost(int64, tag = "11")]
    pub current_battle_stage_master_id: i64,
    #[prost(int64, tag = "90")]
    pub last_update_time: i64,
    #[prost(sint64, tag = "91")]
    pub created_at: i64,
    #[prost(sint64, tag = "92")]
    pub updated_at: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EventStatus {
    InvalidEventStatus = 0,
    EventFinished = 1,
    EventClaimed = 2,
}
impl EventStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EventStatus::InvalidEventStatus => "INVALID_EVENT_STATUS",
            EventStatus::EventFinished => "EVENT_FINISHED",
            EventStatus::EventClaimed => "EVENT_CLAIMED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INVALID_EVENT_STATUS" => Some(Self::InvalidEventStatus),
            "EVENT_FINISHED" => Some(Self::EventFinished),
            "EVENT_CLAIMED" => Some(Self::EventClaimed),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerUnderground {
    #[prost(string, tag = "2")]
    pub player_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    pub captain: i64,
    #[prost(int64, repeated, tag = "21")]
    pub display_battle_character_ids: ::prost::alloc::vec::Vec<i64>,
    #[prost(sint64, tag = "91")]
    pub created_at: i64,
    #[prost(sint64, tag = "92")]
    pub updated_at: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerUndergroundSection {
    #[prost(string, tag = "1")]
    pub player_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub underground_section_master_id: i64,
    #[prost(enumeration = "SectionStatus", tag = "3")]
    pub status: i32,
    #[prost(sint64, tag = "91")]
    pub created_at: i64,
    #[prost(sint64, tag = "92")]
    pub updated_at: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SectionStatus {
    InvalidSectionStatus = 0,
    SectionUnlocked = 1,
    SectionStarted = 2,
}
impl SectionStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SectionStatus::InvalidSectionStatus => "INVALID_SECTION_STATUS",
            SectionStatus::SectionUnlocked => "SECTION_UNLOCKED",
            SectionStatus::SectionStarted => "SECTION_STARTED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INVALID_SECTION_STATUS" => Some(Self::InvalidSectionStatus),
            "SECTION_UNLOCKED" => Some(Self::SectionUnlocked),
            "SECTION_STARTED" => Some(Self::SectionStarted),
            _ => None,
        }
    }
}
