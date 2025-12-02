#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FunctionUnlockMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(enumeration = "FunctionId", tag = "2")]
    pub function_unlock_master_id: i32,
    #[prost(string, repeated, tag = "3")]
    pub require_condition: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FunctionUnlockMasters {
    #[prost(message, repeated, tag = "1")]
    pub function_unlock_masters: ::prost::alloc::vec::Vec<FunctionUnlockMaster>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FunctionId {
    Invalid = 0,
    FunctionCharacterLevelUp = 1,
    FunctionCharacterRankUp = 2,
    FunctionCharacterEquipment = 3,
    FunctionCharacterGradeUp = 4,
    FunctionGachaCharacter = 5,
    FunctionExplore = 6,
    FunctionBackpack = 7,
    FunctionMission = 8,
    FunctionMemoryQuest = 9,
    FunctionDailyQuest = 10,
    FunctionCharacterEngaged = 11,
    FunctionExploreEvent = 12,
    FunctionJukebox = 13,
    FunctionPiano = 14,
    FunctionShop = 15,
    FunctionTeaBreak = 16,
    FunctionGachaMachine = 17,
    FunctionDrinkMaking = 18,
    FunctionCardPlaying = 19,
    FunctionAutoBattle = 20,
    FunctionUnderground = 21,
    FunctionAreaEnter = 22,
    FunctionAreaDinnerRoom = 23,
    FunctionAreaHall = 24,
    FunctionAreaTrainingRoom = 25,
    FunctionNewbieTargetSecondary = 26,
    FunctionMissionDailySecondary = 27,
    FunctionAchievementSecondary = 28,
    FunctionShopBondItemSecondary = 29,
    FunctionShopRaiseItemSecondary = 30,
    FunctionShopEquipmentSecondary = 31,
    FunctionShopCharacterNoteSecondary = 32,
    FunctionShopGermSecondary = 33,
    FunctionShopMusicSecondary = 34,
    FunctionShopGiftSecondary = 35,
    FunctionFollowSecondary = 36,
    FunctionButtonMap = 37,
    FunctionButtonNavigator = 38,
    FunctionButtonGacha = 39,
    FunctionButtonDailyQuest = 40,
    FunctionButtonChapter = 41,
    FunctionNewbieActivity = 42,
    FunctionHomepageSceneRandom = 43,
    FunctionLoginActivity = 50,
}
impl FunctionId {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            FunctionId::Invalid => "INVALID",
            FunctionId::FunctionCharacterLevelUp => "FUNCTION_CHARACTER_LEVEL_UP",
            FunctionId::FunctionCharacterRankUp => "FUNCTION_CHARACTER_RANK_UP",
            FunctionId::FunctionCharacterEquipment => "FUNCTION_CHARACTER_EQUIPMENT",
            FunctionId::FunctionCharacterGradeUp => "FUNCTION_CHARACTER_GRADE_UP",
            FunctionId::FunctionGachaCharacter => "FUNCTION_GACHA_CHARACTER",
            FunctionId::FunctionExplore => "FUNCTION_EXPLORE",
            FunctionId::FunctionBackpack => "FUNCTION_BACKPACK",
            FunctionId::FunctionMission => "FUNCTION_MISSION",
            FunctionId::FunctionMemoryQuest => "FUNCTION_MEMORY_QUEST",
            FunctionId::FunctionDailyQuest => "FUNCTION_DAILY_QUEST",
            FunctionId::FunctionCharacterEngaged => "FUNCTION_CHARACTER_ENGAGED",
            FunctionId::FunctionExploreEvent => "FUNCTION_EXPLORE_EVENT",
            FunctionId::FunctionJukebox => "FUNCTION_JUKEBOX",
            FunctionId::FunctionPiano => "FUNCTION_PIANO",
            FunctionId::FunctionShop => "FUNCTION_SHOP",
            FunctionId::FunctionTeaBreak => "FUNCTION_TEA_BREAK",
            FunctionId::FunctionGachaMachine => "FUNCTION_GACHA_MACHINE",
            FunctionId::FunctionDrinkMaking => "FUNCTION_DRINK_MAKING",
            FunctionId::FunctionCardPlaying => "FUNCTION_CARD_PLAYING",
            FunctionId::FunctionAutoBattle => "FUNCTION_AUTO_BATTLE",
            FunctionId::FunctionUnderground => "FUNCTION_UNDERGROUND",
            FunctionId::FunctionAreaEnter => "FUNCTION_AREA_ENTER",
            FunctionId::FunctionAreaDinnerRoom => "FUNCTION_AREA_DINNER_ROOM",
            FunctionId::FunctionAreaHall => "FUNCTION_AREA_HALL",
            FunctionId::FunctionAreaTrainingRoom => "FUNCTION_AREA_TRAINING_ROOM",
            FunctionId::FunctionNewbieTargetSecondary => {
                "FUNCTION_NEWBIE_TARGET_SECONDARY"
            }
            FunctionId::FunctionMissionDailySecondary => {
                "FUNCTION_MISSION_DAILY_SECONDARY"
            }
            FunctionId::FunctionAchievementSecondary => "FUNCTION_ACHIEVEMENT_SECONDARY",
            FunctionId::FunctionShopBondItemSecondary => {
                "FUNCTION_SHOP_BOND_ITEM_SECONDARY"
            }
            FunctionId::FunctionShopRaiseItemSecondary => {
                "FUNCTION_SHOP_RAISE_ITEM_SECONDARY"
            }
            FunctionId::FunctionShopEquipmentSecondary => {
                "FUNCTION_SHOP_EQUIPMENT_SECONDARY"
            }
            FunctionId::FunctionShopCharacterNoteSecondary => {
                "FUNCTION_SHOP_CHARACTER_NOTE_SECONDARY"
            }
            FunctionId::FunctionShopGermSecondary => "FUNCTION_SHOP_GERM_SECONDARY",
            FunctionId::FunctionShopMusicSecondary => "FUNCTION_SHOP_MUSIC_SECONDARY",
            FunctionId::FunctionShopGiftSecondary => "FUNCTION_SHOP_GIFT_SECONDARY",
            FunctionId::FunctionFollowSecondary => "FUNCTION_FOLLOW_SECONDARY",
            FunctionId::FunctionButtonMap => "FUNCTION_BUTTON_MAP",
            FunctionId::FunctionButtonNavigator => "FUNCTION_BUTTON_NAVIGATOR",
            FunctionId::FunctionButtonGacha => "FUNCTION_BUTTON_GACHA",
            FunctionId::FunctionButtonDailyQuest => "FUNCTION_BUTTON_DAILY_QUEST",
            FunctionId::FunctionButtonChapter => "FUNCTION_BUTTON_CHAPTER",
            FunctionId::FunctionNewbieActivity => "FUNCTION_NEWBIE_ACTIVITY",
            FunctionId::FunctionHomepageSceneRandom => "FUNCTION_HOMEPAGE_SCENE_RANDOM",
            FunctionId::FunctionLoginActivity => "FUNCTION_LOGIN_ACTIVITY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INVALID" => Some(Self::Invalid),
            "FUNCTION_CHARACTER_LEVEL_UP" => Some(Self::FunctionCharacterLevelUp),
            "FUNCTION_CHARACTER_RANK_UP" => Some(Self::FunctionCharacterRankUp),
            "FUNCTION_CHARACTER_EQUIPMENT" => Some(Self::FunctionCharacterEquipment),
            "FUNCTION_CHARACTER_GRADE_UP" => Some(Self::FunctionCharacterGradeUp),
            "FUNCTION_GACHA_CHARACTER" => Some(Self::FunctionGachaCharacter),
            "FUNCTION_EXPLORE" => Some(Self::FunctionExplore),
            "FUNCTION_BACKPACK" => Some(Self::FunctionBackpack),
            "FUNCTION_MISSION" => Some(Self::FunctionMission),
            "FUNCTION_MEMORY_QUEST" => Some(Self::FunctionMemoryQuest),
            "FUNCTION_DAILY_QUEST" => Some(Self::FunctionDailyQuest),
            "FUNCTION_CHARACTER_ENGAGED" => Some(Self::FunctionCharacterEngaged),
            "FUNCTION_EXPLORE_EVENT" => Some(Self::FunctionExploreEvent),
            "FUNCTION_JUKEBOX" => Some(Self::FunctionJukebox),
            "FUNCTION_PIANO" => Some(Self::FunctionPiano),
            "FUNCTION_SHOP" => Some(Self::FunctionShop),
            "FUNCTION_TEA_BREAK" => Some(Self::FunctionTeaBreak),
            "FUNCTION_GACHA_MACHINE" => Some(Self::FunctionGachaMachine),
            "FUNCTION_DRINK_MAKING" => Some(Self::FunctionDrinkMaking),
            "FUNCTION_CARD_PLAYING" => Some(Self::FunctionCardPlaying),
            "FUNCTION_AUTO_BATTLE" => Some(Self::FunctionAutoBattle),
            "FUNCTION_UNDERGROUND" => Some(Self::FunctionUnderground),
            "FUNCTION_AREA_ENTER" => Some(Self::FunctionAreaEnter),
            "FUNCTION_AREA_DINNER_ROOM" => Some(Self::FunctionAreaDinnerRoom),
            "FUNCTION_AREA_HALL" => Some(Self::FunctionAreaHall),
            "FUNCTION_AREA_TRAINING_ROOM" => Some(Self::FunctionAreaTrainingRoom),
            "FUNCTION_NEWBIE_TARGET_SECONDARY" => {
                Some(Self::FunctionNewbieTargetSecondary)
            }
            "FUNCTION_MISSION_DAILY_SECONDARY" => {
                Some(Self::FunctionMissionDailySecondary)
            }
            "FUNCTION_ACHIEVEMENT_SECONDARY" => Some(Self::FunctionAchievementSecondary),
            "FUNCTION_SHOP_BOND_ITEM_SECONDARY" => {
                Some(Self::FunctionShopBondItemSecondary)
            }
            "FUNCTION_SHOP_RAISE_ITEM_SECONDARY" => {
                Some(Self::FunctionShopRaiseItemSecondary)
            }
            "FUNCTION_SHOP_EQUIPMENT_SECONDARY" => {
                Some(Self::FunctionShopEquipmentSecondary)
            }
            "FUNCTION_SHOP_CHARACTER_NOTE_SECONDARY" => {
                Some(Self::FunctionShopCharacterNoteSecondary)
            }
            "FUNCTION_SHOP_GERM_SECONDARY" => Some(Self::FunctionShopGermSecondary),
            "FUNCTION_SHOP_MUSIC_SECONDARY" => Some(Self::FunctionShopMusicSecondary),
            "FUNCTION_SHOP_GIFT_SECONDARY" => Some(Self::FunctionShopGiftSecondary),
            "FUNCTION_FOLLOW_SECONDARY" => Some(Self::FunctionFollowSecondary),
            "FUNCTION_BUTTON_MAP" => Some(Self::FunctionButtonMap),
            "FUNCTION_BUTTON_NAVIGATOR" => Some(Self::FunctionButtonNavigator),
            "FUNCTION_BUTTON_GACHA" => Some(Self::FunctionButtonGacha),
            "FUNCTION_BUTTON_DAILY_QUEST" => Some(Self::FunctionButtonDailyQuest),
            "FUNCTION_BUTTON_CHAPTER" => Some(Self::FunctionButtonChapter),
            "FUNCTION_NEWBIE_ACTIVITY" => Some(Self::FunctionNewbieActivity),
            "FUNCTION_HOMEPAGE_SCENE_RANDOM" => Some(Self::FunctionHomepageSceneRandom),
            "FUNCTION_LOGIN_ACTIVITY" => Some(Self::FunctionLoginActivity),
            _ => None,
        }
    }
}
