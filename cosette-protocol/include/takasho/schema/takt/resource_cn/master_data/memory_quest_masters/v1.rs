#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MemoryQuestMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub memory_quest_master_id: i64,
    #[prost(int64, tag = "3")]
    pub pre_memory_quest_master_id: i64,
    #[prost(int64, tag = "4")]
    pub chapter: i64,
    #[prost(int64, tag = "5")]
    pub require_chapter_master_id: i64,
    #[prost(int64, tag = "6")]
    pub quest_id: i64,
    #[prost(enumeration = "MemoryQuestType", tag = "11")]
    pub memory_quest_type: i32,
    #[prost(int64, tag = "12")]
    pub use_stamina: i64,
    #[prost(int64, tag = "13")]
    pub battle_master_id: i64,
    #[prost(int64, tag = "14")]
    pub limit_daily_count: i64,
    #[prost(enumeration = "MemoryQuestSort", tag = "15")]
    pub memory_quest_sort: i32,
    #[prost(int64, tag = "21")]
    pub round_num: i64,
    #[prost(int64, tag = "22")]
    pub death_num: i64,
    #[prost(int64, tag = "23")]
    pub exp_player: i64,
    #[prost(int64, tag = "24")]
    pub exp_battle_character: i64,
    #[prost(int64, tag = "25")]
    pub game_money: i64,
    #[prost(string, repeated, tag = "26")]
    pub first_clear_reward_contents: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    #[prost(int64, repeated, tag = "27")]
    pub drop_master_ids: ::prost::alloc::vec::Vec<i64>,
    #[prost(string, repeated, tag = "29")]
    pub skip_reward_contents: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MemoryQuestMasters {
    #[prost(message, repeated, tag = "1")]
    pub memory_quest_masters: ::prost::alloc::vec::Vec<MemoryQuestMaster>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MemoryQuestType {
    InvalidBattleType = 0,
    Normal = 1,
    Difficult = 2,
}
impl MemoryQuestType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MemoryQuestType::InvalidBattleType => "INVALID_BATTLE_TYPE",
            MemoryQuestType::Normal => "NORMAL",
            MemoryQuestType::Difficult => "DIFFICULT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INVALID_BATTLE_TYPE" => Some(Self::InvalidBattleType),
            "NORMAL" => Some(Self::Normal),
            "DIFFICULT" => Some(Self::Difficult),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MemoryQuestSort {
    InvalidBattleSort = 0,
    TrivialQuest = 1,
    CrucialQuest = 2,
}
impl MemoryQuestSort {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MemoryQuestSort::InvalidBattleSort => "INVALID_BATTLE_SORT",
            MemoryQuestSort::TrivialQuest => "TRIVIAL_QUEST",
            MemoryQuestSort::CrucialQuest => "CRUCIAL_QUEST",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INVALID_BATTLE_SORT" => Some(Self::InvalidBattleSort),
            "TRIVIAL_QUEST" => Some(Self::TrivialQuest),
            "CRUCIAL_QUEST" => Some(Self::CrucialQuest),
            _ => None,
        }
    }
}
