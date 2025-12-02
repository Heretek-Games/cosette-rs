#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Limit {
    #[prost(int64, tag = "1")]
    pub count: i64,
    #[prost(sint64, tag = "2")]
    pub next_cycle_start_at: i64,
    #[prost(sint64, tag = "3")]
    pub updated_at: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LimitType {
    InvalidLimit = 0,
    LimitForever = 1,
    LimitPerDay = 2,
    LimitPerWeek = 3,
    LimitPerMonth = 4,
    LimitPerNaturalDay = 5,
}
impl LimitType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LimitType::InvalidLimit => "INVALID_LIMIT",
            LimitType::LimitForever => "LIMIT_FOREVER",
            LimitType::LimitPerDay => "LIMIT_PER_DAY",
            LimitType::LimitPerWeek => "LIMIT_PER_WEEK",
            LimitType::LimitPerMonth => "LIMIT_PER_MONTH",
            LimitType::LimitPerNaturalDay => "LIMIT_PER_NATURAL_DAY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INVALID_LIMIT" => Some(Self::InvalidLimit),
            "LIMIT_FOREVER" => Some(Self::LimitForever),
            "LIMIT_PER_DAY" => Some(Self::LimitPerDay),
            "LIMIT_PER_WEEK" => Some(Self::LimitPerWeek),
            "LIMIT_PER_MONTH" => Some(Self::LimitPerMonth),
            "LIMIT_PER_NATURAL_DAY" => Some(Self::LimitPerNaturalDay),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TriggerType {
    InvalidTrigger = 0,
    TriggerPlayerLevelUp = 1,
    TriggerCharacterBondLevelUp = 2,
    TriggerUseItem = 3,
    TriggerFinishPveChapter = 4,
    TriggerExploreFinishEvent = 5,
    TriggerExploreFinishFreeExplore = 6,
}
impl TriggerType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TriggerType::InvalidTrigger => "INVALID_TRIGGER",
            TriggerType::TriggerPlayerLevelUp => "TRIGGER_PLAYER_LEVEL_UP",
            TriggerType::TriggerCharacterBondLevelUp => "TRIGGER_CHARACTER_BOND_LEVEL_UP",
            TriggerType::TriggerUseItem => "TRIGGER_USE_ITEM",
            TriggerType::TriggerFinishPveChapter => "TRIGGER_FINISH_PVE_CHAPTER",
            TriggerType::TriggerExploreFinishEvent => "TRIGGER_EXPLORE_FINISH_EVENT",
            TriggerType::TriggerExploreFinishFreeExplore => {
                "TRIGGER_EXPLORE_FINISH_FREE_EXPLORE"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INVALID_TRIGGER" => Some(Self::InvalidTrigger),
            "TRIGGER_PLAYER_LEVEL_UP" => Some(Self::TriggerPlayerLevelUp),
            "TRIGGER_CHARACTER_BOND_LEVEL_UP" => Some(Self::TriggerCharacterBondLevelUp),
            "TRIGGER_USE_ITEM" => Some(Self::TriggerUseItem),
            "TRIGGER_FINISH_PVE_CHAPTER" => Some(Self::TriggerFinishPveChapter),
            "TRIGGER_EXPLORE_FINISH_EVENT" => Some(Self::TriggerExploreFinishEvent),
            "TRIGGER_EXPLORE_FINISH_FREE_EXPLORE" => {
                Some(Self::TriggerExploreFinishFreeExplore)
            }
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Attribute {
    InvalidAttribute = 0,
    HpAttribute = 1,
    DexAttribute = 2,
    AtkAttribute = 3,
    DefAttribute = 4,
    IntAtkAttribute = 5,
    IntResAttribute = 6,
    CrtAttribute = 7,
    CrdAttribute = 8,
    AcrtAttribute = 9,
    CrdrAttribute = 10,
    ApAttribute = 11,
}
impl Attribute {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Attribute::InvalidAttribute => "INVALID_ATTRIBUTE",
            Attribute::HpAttribute => "HP_ATTRIBUTE",
            Attribute::DexAttribute => "DEX_ATTRIBUTE",
            Attribute::AtkAttribute => "ATK_ATTRIBUTE",
            Attribute::DefAttribute => "DEF_ATTRIBUTE",
            Attribute::IntAtkAttribute => "INT_ATK_ATTRIBUTE",
            Attribute::IntResAttribute => "INT_RES_ATTRIBUTE",
            Attribute::CrtAttribute => "CRT_ATTRIBUTE",
            Attribute::CrdAttribute => "CRD_ATTRIBUTE",
            Attribute::AcrtAttribute => "ACRT_ATTRIBUTE",
            Attribute::CrdrAttribute => "CRDR_ATTRIBUTE",
            Attribute::ApAttribute => "AP_ATTRIBUTE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INVALID_ATTRIBUTE" => Some(Self::InvalidAttribute),
            "HP_ATTRIBUTE" => Some(Self::HpAttribute),
            "DEX_ATTRIBUTE" => Some(Self::DexAttribute),
            "ATK_ATTRIBUTE" => Some(Self::AtkAttribute),
            "DEF_ATTRIBUTE" => Some(Self::DefAttribute),
            "INT_ATK_ATTRIBUTE" => Some(Self::IntAtkAttribute),
            "INT_RES_ATTRIBUTE" => Some(Self::IntResAttribute),
            "CRT_ATTRIBUTE" => Some(Self::CrtAttribute),
            "CRD_ATTRIBUTE" => Some(Self::CrdAttribute),
            "ACRT_ATTRIBUTE" => Some(Self::AcrtAttribute),
            "CRDR_ATTRIBUTE" => Some(Self::CrdrAttribute),
            "AP_ATTRIBUTE" => Some(Self::ApAttribute),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ConditionType {
    InvalidCondition = 0,
    ConditionPlayerLevel = 1,
    ConditionCharacterBondLevel = 2,
    ConditionExploreFinishEvent = 3,
    ConditionExploreFinishFreeExplore = 4,
    ConditionDatetime = 5,
    ConditionPveFinishChapter = 6,
    ConditionReachDays = 7,
    ConditionCrusadeTreasureAvailable = 8,
    ConditionItemCount = 9,
    ConditionTutorial = 10,
    ConditionCrusadeFinishedMapId = 11,
    ConditionBattleAchievement = 12,
    ConditionBattleCharacterEngage = 13,
    ConditionBattleCharacterRank = 14,
    ConditionMasterSectionPoints = 15,
    ConditionUndergroundFinishEvent = 201,
    ConditionUndergroundFinishSection = 202,
    ConditionPhonemeAttribute = 301,
    ConditionCrusadeLevel = 401,
    ConditionClosedAfterPlayerCreatedDaysExceed = 501,
}
impl ConditionType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ConditionType::InvalidCondition => "INVALID_CONDITION",
            ConditionType::ConditionPlayerLevel => "CONDITION_PLAYER_LEVEL",
            ConditionType::ConditionCharacterBondLevel => {
                "CONDITION_CHARACTER_BOND_LEVEL"
            }
            ConditionType::ConditionExploreFinishEvent => {
                "CONDITION_EXPLORE_FINISH_EVENT"
            }
            ConditionType::ConditionExploreFinishFreeExplore => {
                "CONDITION_EXPLORE_FINISH_FREE_EXPLORE"
            }
            ConditionType::ConditionDatetime => "CONDITION_DATETIME",
            ConditionType::ConditionPveFinishChapter => "CONDITION_PVE_FINISH_CHAPTER",
            ConditionType::ConditionReachDays => "CONDITION_REACH_DAYS",
            ConditionType::ConditionCrusadeTreasureAvailable => {
                "CONDITION_CRUSADE_TREASURE_AVAILABLE"
            }
            ConditionType::ConditionItemCount => "CONDITION_ITEM_COUNT",
            ConditionType::ConditionTutorial => "CONDITION_TUTORIAL",
            ConditionType::ConditionCrusadeFinishedMapId => {
                "CONDITION_CRUSADE_FINISHED_MAP_ID"
            }
            ConditionType::ConditionBattleAchievement => "CONDITION_BATTLE_ACHIEVEMENT",
            ConditionType::ConditionBattleCharacterEngage => {
                "CONDITION_BATTLE_CHARACTER_ENGAGE"
            }
            ConditionType::ConditionBattleCharacterRank => {
                "CONDITION_BATTLE_CHARACTER_RANK"
            }
            ConditionType::ConditionMasterSectionPoints => {
                "CONDITION_MASTER_SECTION_POINTS"
            }
            ConditionType::ConditionUndergroundFinishEvent => {
                "CONDITION_UNDERGROUND_FINISH_EVENT"
            }
            ConditionType::ConditionUndergroundFinishSection => {
                "CONDITION_UNDERGROUND_FINISH_SECTION"
            }
            ConditionType::ConditionPhonemeAttribute => "CONDITION_PHONEME_ATTRIBUTE",
            ConditionType::ConditionCrusadeLevel => "CONDITION_CRUSADE_LEVEL",
            ConditionType::ConditionClosedAfterPlayerCreatedDaysExceed => {
                "CONDITION_CLOSED_AFTER_PLAYER_CREATED_DAYS_EXCEED"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INVALID_CONDITION" => Some(Self::InvalidCondition),
            "CONDITION_PLAYER_LEVEL" => Some(Self::ConditionPlayerLevel),
            "CONDITION_CHARACTER_BOND_LEVEL" => Some(Self::ConditionCharacterBondLevel),
            "CONDITION_EXPLORE_FINISH_EVENT" => Some(Self::ConditionExploreFinishEvent),
            "CONDITION_EXPLORE_FINISH_FREE_EXPLORE" => {
                Some(Self::ConditionExploreFinishFreeExplore)
            }
            "CONDITION_DATETIME" => Some(Self::ConditionDatetime),
            "CONDITION_PVE_FINISH_CHAPTER" => Some(Self::ConditionPveFinishChapter),
            "CONDITION_REACH_DAYS" => Some(Self::ConditionReachDays),
            "CONDITION_CRUSADE_TREASURE_AVAILABLE" => {
                Some(Self::ConditionCrusadeTreasureAvailable)
            }
            "CONDITION_ITEM_COUNT" => Some(Self::ConditionItemCount),
            "CONDITION_TUTORIAL" => Some(Self::ConditionTutorial),
            "CONDITION_CRUSADE_FINISHED_MAP_ID" => {
                Some(Self::ConditionCrusadeFinishedMapId)
            }
            "CONDITION_BATTLE_ACHIEVEMENT" => Some(Self::ConditionBattleAchievement),
            "CONDITION_BATTLE_CHARACTER_ENGAGE" => {
                Some(Self::ConditionBattleCharacterEngage)
            }
            "CONDITION_BATTLE_CHARACTER_RANK" => Some(Self::ConditionBattleCharacterRank),
            "CONDITION_MASTER_SECTION_POINTS" => Some(Self::ConditionMasterSectionPoints),
            "CONDITION_UNDERGROUND_FINISH_EVENT" => {
                Some(Self::ConditionUndergroundFinishEvent)
            }
            "CONDITION_UNDERGROUND_FINISH_SECTION" => {
                Some(Self::ConditionUndergroundFinishSection)
            }
            "CONDITION_PHONEME_ATTRIBUTE" => Some(Self::ConditionPhonemeAttribute),
            "CONDITION_CRUSADE_LEVEL" => Some(Self::ConditionCrusadeLevel),
            "CONDITION_CLOSED_AFTER_PLAYER_CREATED_DAYS_EXCEED" => {
                Some(Self::ConditionClosedAfterPlayerCreatedDaysExceed)
            }
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Rewards {
    #[prost(message, repeated, tag = "1")]
    pub rewards: ::prost::alloc::vec::Vec<rewards::RewardsEntry>,
    #[prost(message, repeated, tag = "2")]
    pub first_clear_rewards: ::prost::alloc::vec::Vec<rewards::FirstClearRewardsEntry>,
}
/// Nested message and enum types in `Rewards`.
pub mod rewards {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RewardsEntry {
        #[prost(int64, tag = "1")]
        pub key: i64,
        #[prost(int64, tag = "2")]
        pub value: i64,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FirstClearRewardsEntry {
        #[prost(int64, tag = "1")]
        pub key: i64,
        #[prost(int64, tag = "2")]
        pub value: i64,
    }
}
