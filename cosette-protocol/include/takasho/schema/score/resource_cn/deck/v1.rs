#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerDeck {
    #[prost(string, tag = "1")]
    pub player_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub deck_id: ::prost::alloc::string::String,
    #[prost(int64, repeated, tag = "3")]
    pub battle_character_master_ids: ::prost::alloc::vec::Vec<i64>,
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
    #[prost(enumeration = "DeckType", tag = "5")]
    pub deck_type: i32,
    #[prost(int64, tag = "6")]
    pub leader_id: i64,
    #[prost(sint64, tag = "91")]
    pub created_at: i64,
    #[prost(sint64, tag = "92")]
    pub updated_at: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DeckType {
    InvalidDeckType = 0,
    PveType = 1,
    MemoryQuestType = 2,
    DailyQuestType = 3,
    ExploreEventType = 4,
    UndergroundType = 5,
    CrusadeType = 6,
    CrusadeActivityType = 7,
    BattleActivityType = 8,
    MasterSectionType = 9,
}
impl DeckType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DeckType::InvalidDeckType => "INVALID_DECK_TYPE",
            DeckType::PveType => "PVE_TYPE",
            DeckType::MemoryQuestType => "MEMORY_QUEST_TYPE",
            DeckType::DailyQuestType => "DAILY_QUEST_TYPE",
            DeckType::ExploreEventType => "EXPLORE_EVENT_TYPE",
            DeckType::UndergroundType => "UNDERGROUND_TYPE",
            DeckType::CrusadeType => "CRUSADE_TYPE",
            DeckType::CrusadeActivityType => "CRUSADE_ACTIVITY_TYPE",
            DeckType::BattleActivityType => "BATTLE_ACTIVITY_TYPE",
            DeckType::MasterSectionType => "MASTER_SECTION_TYPE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INVALID_DECK_TYPE" => Some(Self::InvalidDeckType),
            "PVE_TYPE" => Some(Self::PveType),
            "MEMORY_QUEST_TYPE" => Some(Self::MemoryQuestType),
            "DAILY_QUEST_TYPE" => Some(Self::DailyQuestType),
            "EXPLORE_EVENT_TYPE" => Some(Self::ExploreEventType),
            "UNDERGROUND_TYPE" => Some(Self::UndergroundType),
            "CRUSADE_TYPE" => Some(Self::CrusadeType),
            "CRUSADE_ACTIVITY_TYPE" => Some(Self::CrusadeActivityType),
            "BATTLE_ACTIVITY_TYPE" => Some(Self::BattleActivityType),
            "MASTER_SECTION_TYPE" => Some(Self::MasterSectionType),
            _ => None,
        }
    }
}
