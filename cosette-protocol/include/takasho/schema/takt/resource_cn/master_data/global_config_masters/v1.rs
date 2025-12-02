#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GlobalConfigMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(enumeration = "Id", tag = "2")]
    pub global_config_master_id: i32,
    #[prost(int64, tag = "3")]
    pub data: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GlobalConfigMasters {
    #[prost(message, repeated, tag = "1")]
    pub global_config_masters: ::prost::alloc::vec::Vec<GlobalConfigMaster>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Id {
    Invalid = 0,
    PlayerChangeNameCost = 1,
    MaxStamina = 2,
    StaminaRecoveryCycle = 3,
    DateLine = 4,
    MaxItemStacks = 5,
    MaxEquipments = 6,
    CreatPlayerNameTutorialStep = 9,
    PlayerCanSetBadgeLimit = 10,
    BattleCharacterLevelExceedPlayerLevelLimit = 101,
    BattleCharacterNotesExchangeRate = 102,
    BattleCharacterLevelUpGoldCost = 103,
    BattleCharacterRankUpGoldCost = 104,
    BattleCharacterFavoriteBondItemExtraPercentage = 105,
    BattleCharacterMaxSkillSlot = 106,
    EquipmentLevelUpGoldCost = 202,
    EquipmentGradeUpGoldCost = 203,
    EquipmentRankUpGoldCost = 204,
    EquipmentNDecomposeRate = 211,
    EquipmentRDecomposeRate = 212,
    EquipmentSrDecomposeRate = 213,
    EquipmentSsrDecomposeRate = 214,
    DailyQuestSkipTimesLimit = 301,
    GachaDailyLimitCostContentId = 302,
    DailyQuestSweepDuration = 303,
    DailyQuestSweepTurnLimit = 304,
    ExploreDailyEventsLimit = 401,
    ExploreDailyBattleCharactersLimit = 402,
    ExploreDailyBattleCharactersRemainHours = 403,
    ExploreDailyInvitationLimit = 404,
    ExploreStaminaToEnergyRatio = 405,
    MaxEnergy = 406,
    ExploreDailyEventNumber = 407,
    ExploreDailyEventRefreshHours = 408,
    ExploreFavoriteGiftBond = 409,
    ExploreLikeGiftBond = 410,
    ExploreNormalGiftBond = 411,
    ExploreHateGiftBond = 412,
    ExploreTutorialExplorePointMasterId = 413,
    ExploreCoinDailyLimit = 415,
    ExploreCoinDailyLimitMax = 416,
    TeabreakFavoriteItem = 417,
    TeabreakLikeItem = 418,
    TeabreakNormalItem = 419,
    DrinkMakingARequireMark = 420,
    DrinkMakingSRequireMark = 421,
    DrinkMakingAccessoriesCount = 422,
    DailyKeywordMaxDropCount = 423,
    DailyKeywordGuaranteedCount = 424,
    MailBoxCapacity = 501,
    GiftCodeMailTemplateId = 502,
    RechargeRebateTemplateId = 503,
    WebActivityMailTemplateId = 504,
    CrusadeRestartTime = 606,
    CrusadeBattleWinRecoverAp = 608,
    CrusadeMaxTreasureChoice = 609,
    CrusadeCoinTransPercent = 610,
    CrusadeCoinInitLimit = 611,
    CrusadeFreeGemLimit = 612,
    CrusadeRewardsMailTemplatedId = 613,
    CrusadeSweepMaxTimes = 614,
    CrusadeSweepMinSeconds = 615,
    MaxPhonemes = 701,
    MaxPhonemeBuffs = 702,
    PhonemeLevelUpGoldCost = 703,
    PhonemeGradeUpGoldCost = 704,
    PhonemeDecomposeRefundPercentage = 705,
    PhonemeDecomposeRefundItem = 706,
    PhonemeLevelupPercentage = 707,
    PhonemeDecomposeType = 708,
    GachaGemCost = 9999,
    BattleCharacterMaxRank = 801,
    HomepageSceneDefault = 901,
    PlayerMonthlyPassLimit = 1001,
    PlayerPopGiftQueueCapacity = 1003,
    AppleStoreGiftCodeShop = 1004,
    AppleStoreGiftCodeMailTemplate = 1005,
    MasterSectionDuration = 1101,
    MasterSectionStartTime = 1102,
    MasterSectionStartMasterId = 1103,
    MasterSectionCoolingDownTurns = 1104,
    MasterSectionPolynomialCoefficient = 1204,
    MasterSectionPolynomialBaseCoefficient = 1205,
}
impl Id {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Id::Invalid => "INVALID",
            Id::PlayerChangeNameCost => "PLAYER_CHANGE_NAME_COST",
            Id::MaxStamina => "MAX_STAMINA",
            Id::StaminaRecoveryCycle => "STAMINA_RECOVERY_CYCLE",
            Id::DateLine => "DATE_LINE",
            Id::MaxItemStacks => "MAX_ITEM_STACKS",
            Id::MaxEquipments => "MAX_EQUIPMENTS",
            Id::CreatPlayerNameTutorialStep => "CREAT_PLAYER_NAME_TUTORIAL_STEP",
            Id::PlayerCanSetBadgeLimit => "PLAYER_CAN_SET_BADGE_LIMIT",
            Id::BattleCharacterLevelExceedPlayerLevelLimit => {
                "BATTLE_CHARACTER_LEVEL_EXCEED_PLAYER_LEVEL_LIMIT"
            }
            Id::BattleCharacterNotesExchangeRate => {
                "BATTLE_CHARACTER_NOTES_EXCHANGE_RATE"
            }
            Id::BattleCharacterLevelUpGoldCost => "BATTLE_CHARACTER_LEVEL_UP_GOLD_COST",
            Id::BattleCharacterRankUpGoldCost => "BATTLE_CHARACTER_RANK_UP_GOLD_COST",
            Id::BattleCharacterFavoriteBondItemExtraPercentage => {
                "BATTLE_CHARACTER_FAVORITE_BOND_ITEM_EXTRA_PERCENTAGE"
            }
            Id::BattleCharacterMaxSkillSlot => "BATTLE_CHARACTER_MAX_SKILL_SLOT",
            Id::EquipmentLevelUpGoldCost => "EQUIPMENT_LEVEL_UP_GOLD_COST",
            Id::EquipmentGradeUpGoldCost => "EQUIPMENT_GRADE_UP_GOLD_COST",
            Id::EquipmentRankUpGoldCost => "EQUIPMENT_RANK_UP_GOLD_COST",
            Id::EquipmentNDecomposeRate => "EQUIPMENT_N_DECOMPOSE_RATE",
            Id::EquipmentRDecomposeRate => "EQUIPMENT_R_DECOMPOSE_RATE",
            Id::EquipmentSrDecomposeRate => "EQUIPMENT_SR_DECOMPOSE_RATE",
            Id::EquipmentSsrDecomposeRate => "EQUIPMENT_SSR_DECOMPOSE_RATE",
            Id::DailyQuestSkipTimesLimit => "DAILY_QUEST_SKIP_TIMES_LIMIT",
            Id::GachaDailyLimitCostContentId => "GACHA_DAILY_LIMIT_COST_CONTENT_ID",
            Id::DailyQuestSweepDuration => "DAILY_QUEST_SWEEP_DURATION",
            Id::DailyQuestSweepTurnLimit => "DAILY_QUEST_SWEEP_TURN_LIMIT",
            Id::ExploreDailyEventsLimit => "EXPLORE_DAILY_EVENTS_LIMIT",
            Id::ExploreDailyBattleCharactersLimit => {
                "EXPLORE_DAILY_BATTLE_CHARACTERS_LIMIT"
            }
            Id::ExploreDailyBattleCharactersRemainHours => {
                "EXPLORE_DAILY_BATTLE_CHARACTERS_REMAIN_HOURS"
            }
            Id::ExploreDailyInvitationLimit => "EXPLORE_DAILY_INVITATION_LIMIT",
            Id::ExploreStaminaToEnergyRatio => "EXPLORE_STAMINA_TO_ENERGY_RATIO",
            Id::MaxEnergy => "MAX_ENERGY",
            Id::ExploreDailyEventNumber => "EXPLORE_DAILY_EVENT_NUMBER",
            Id::ExploreDailyEventRefreshHours => "EXPLORE_DAILY_EVENT_REFRESH_HOURS",
            Id::ExploreFavoriteGiftBond => "EXPLORE_FAVORITE_GIFT_BOND",
            Id::ExploreLikeGiftBond => "EXPLORE_LIKE_GIFT_BOND",
            Id::ExploreNormalGiftBond => "EXPLORE_NORMAL_GIFT_BOND",
            Id::ExploreHateGiftBond => "EXPLORE_HATE_GIFT_BOND",
            Id::ExploreTutorialExplorePointMasterId => {
                "EXPLORE_TUTORIAL_EXPLORE_POINT_MASTER_ID"
            }
            Id::ExploreCoinDailyLimit => "EXPLORE_COIN_DAILY_LIMIT",
            Id::ExploreCoinDailyLimitMax => "EXPLORE_COIN_DAILY_LIMIT_MAX",
            Id::TeabreakFavoriteItem => "TEABREAK_FAVORITE_ITEM",
            Id::TeabreakLikeItem => "TEABREAK_LIKE_ITEM",
            Id::TeabreakNormalItem => "TEABREAK_NORMAL_ITEM",
            Id::DrinkMakingARequireMark => "DRINK_MAKING_A_REQUIRE_MARK",
            Id::DrinkMakingSRequireMark => "DRINK_MAKING_S_REQUIRE_MARK",
            Id::DrinkMakingAccessoriesCount => "DRINK_MAKING_ACCESSORIES_COUNT",
            Id::DailyKeywordMaxDropCount => "DAILY_KEYWORD_MAX_DROP_COUNT",
            Id::DailyKeywordGuaranteedCount => "DAILY_KEYWORD_GUARANTEED_COUNT",
            Id::MailBoxCapacity => "MAIL_BOX_CAPACITY",
            Id::GiftCodeMailTemplateId => "GIFT_CODE_MAIL_TEMPLATE_ID",
            Id::RechargeRebateTemplateId => "RECHARGE_REBATE_TEMPLATE_ID",
            Id::WebActivityMailTemplateId => "WEB_ACTIVITY_MAIL_TEMPLATE_ID",
            Id::CrusadeRestartTime => "CRUSADE_RESTART_TIME",
            Id::CrusadeBattleWinRecoverAp => "CRUSADE_BATTLE_WIN_RECOVER_AP",
            Id::CrusadeMaxTreasureChoice => "CRUSADE_MAX_TREASURE_CHOICE",
            Id::CrusadeCoinTransPercent => "CRUSADE_COIN_TRANS_PERCENT",
            Id::CrusadeCoinInitLimit => "CRUSADE_COIN_INIT_LIMIT",
            Id::CrusadeFreeGemLimit => "CRUSADE_FREE_GEM_LIMIT",
            Id::CrusadeRewardsMailTemplatedId => "CRUSADE_REWARDS_MAIL_TEMPLATED_ID",
            Id::CrusadeSweepMaxTimes => "CRUSADE_SWEEP_MAX_TIMES",
            Id::CrusadeSweepMinSeconds => "CRUSADE_SWEEP_MIN_SECONDS",
            Id::MaxPhonemes => "MAX_PHONEMES",
            Id::MaxPhonemeBuffs => "MAX_PHONEME_BUFFS",
            Id::PhonemeLevelUpGoldCost => "PHONEME_LEVEL_UP_GOLD_COST",
            Id::PhonemeGradeUpGoldCost => "PHONEME_GRADE_UP_GOLD_COST",
            Id::PhonemeDecomposeRefundPercentage => "PHONEME_DECOMPOSE_REFUND_PERCENTAGE",
            Id::PhonemeDecomposeRefundItem => "PHONEME_DECOMPOSE_REFUND_ITEM",
            Id::PhonemeLevelupPercentage => "PHONEME_LEVELUP_PERCENTAGE",
            Id::PhonemeDecomposeType => "PHONEME_DECOMPOSE_TYPE",
            Id::GachaGemCost => "GACHA_GEM_COST",
            Id::BattleCharacterMaxRank => "BATTLE_CHARACTER_MAX_RANK",
            Id::HomepageSceneDefault => "HOMEPAGE_SCENE_DEFAULT",
            Id::PlayerMonthlyPassLimit => "PLAYER_MONTHLY_PASS_LIMIT",
            Id::PlayerPopGiftQueueCapacity => "PLAYER_POP_GIFT_QUEUE_CAPACITY",
            Id::AppleStoreGiftCodeShop => "APPLE_STORE_GIFT_CODE_SHOP",
            Id::AppleStoreGiftCodeMailTemplate => "APPLE_STORE_GIFT_CODE_MAIL_TEMPLATE",
            Id::MasterSectionDuration => "MASTER_SECTION_DURATION",
            Id::MasterSectionStartTime => "MASTER_SECTION_START_TIME",
            Id::MasterSectionStartMasterId => "MASTER_SECTION_START_MASTER_ID",
            Id::MasterSectionCoolingDownTurns => "MASTER_SECTION_COOLING_DOWN_TURNS",
            Id::MasterSectionPolynomialCoefficient => {
                "MASTER_SECTION_POLYNOMIAL_COEFFICIENT"
            }
            Id::MasterSectionPolynomialBaseCoefficient => {
                "MASTER_SECTION_POLYNOMIAL_BASE_COEFFICIENT"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INVALID" => Some(Self::Invalid),
            "PLAYER_CHANGE_NAME_COST" => Some(Self::PlayerChangeNameCost),
            "MAX_STAMINA" => Some(Self::MaxStamina),
            "STAMINA_RECOVERY_CYCLE" => Some(Self::StaminaRecoveryCycle),
            "DATE_LINE" => Some(Self::DateLine),
            "MAX_ITEM_STACKS" => Some(Self::MaxItemStacks),
            "MAX_EQUIPMENTS" => Some(Self::MaxEquipments),
            "CREAT_PLAYER_NAME_TUTORIAL_STEP" => Some(Self::CreatPlayerNameTutorialStep),
            "PLAYER_CAN_SET_BADGE_LIMIT" => Some(Self::PlayerCanSetBadgeLimit),
            "BATTLE_CHARACTER_LEVEL_EXCEED_PLAYER_LEVEL_LIMIT" => {
                Some(Self::BattleCharacterLevelExceedPlayerLevelLimit)
            }
            "BATTLE_CHARACTER_NOTES_EXCHANGE_RATE" => {
                Some(Self::BattleCharacterNotesExchangeRate)
            }
            "BATTLE_CHARACTER_LEVEL_UP_GOLD_COST" => {
                Some(Self::BattleCharacterLevelUpGoldCost)
            }
            "BATTLE_CHARACTER_RANK_UP_GOLD_COST" => {
                Some(Self::BattleCharacterRankUpGoldCost)
            }
            "BATTLE_CHARACTER_FAVORITE_BOND_ITEM_EXTRA_PERCENTAGE" => {
                Some(Self::BattleCharacterFavoriteBondItemExtraPercentage)
            }
            "BATTLE_CHARACTER_MAX_SKILL_SLOT" => Some(Self::BattleCharacterMaxSkillSlot),
            "EQUIPMENT_LEVEL_UP_GOLD_COST" => Some(Self::EquipmentLevelUpGoldCost),
            "EQUIPMENT_GRADE_UP_GOLD_COST" => Some(Self::EquipmentGradeUpGoldCost),
            "EQUIPMENT_RANK_UP_GOLD_COST" => Some(Self::EquipmentRankUpGoldCost),
            "EQUIPMENT_N_DECOMPOSE_RATE" => Some(Self::EquipmentNDecomposeRate),
            "EQUIPMENT_R_DECOMPOSE_RATE" => Some(Self::EquipmentRDecomposeRate),
            "EQUIPMENT_SR_DECOMPOSE_RATE" => Some(Self::EquipmentSrDecomposeRate),
            "EQUIPMENT_SSR_DECOMPOSE_RATE" => Some(Self::EquipmentSsrDecomposeRate),
            "DAILY_QUEST_SKIP_TIMES_LIMIT" => Some(Self::DailyQuestSkipTimesLimit),
            "GACHA_DAILY_LIMIT_COST_CONTENT_ID" => {
                Some(Self::GachaDailyLimitCostContentId)
            }
            "DAILY_QUEST_SWEEP_DURATION" => Some(Self::DailyQuestSweepDuration),
            "DAILY_QUEST_SWEEP_TURN_LIMIT" => Some(Self::DailyQuestSweepTurnLimit),
            "EXPLORE_DAILY_EVENTS_LIMIT" => Some(Self::ExploreDailyEventsLimit),
            "EXPLORE_DAILY_BATTLE_CHARACTERS_LIMIT" => {
                Some(Self::ExploreDailyBattleCharactersLimit)
            }
            "EXPLORE_DAILY_BATTLE_CHARACTERS_REMAIN_HOURS" => {
                Some(Self::ExploreDailyBattleCharactersRemainHours)
            }
            "EXPLORE_DAILY_INVITATION_LIMIT" => Some(Self::ExploreDailyInvitationLimit),
            "EXPLORE_STAMINA_TO_ENERGY_RATIO" => Some(Self::ExploreStaminaToEnergyRatio),
            "MAX_ENERGY" => Some(Self::MaxEnergy),
            "EXPLORE_DAILY_EVENT_NUMBER" => Some(Self::ExploreDailyEventNumber),
            "EXPLORE_DAILY_EVENT_REFRESH_HOURS" => {
                Some(Self::ExploreDailyEventRefreshHours)
            }
            "EXPLORE_FAVORITE_GIFT_BOND" => Some(Self::ExploreFavoriteGiftBond),
            "EXPLORE_LIKE_GIFT_BOND" => Some(Self::ExploreLikeGiftBond),
            "EXPLORE_NORMAL_GIFT_BOND" => Some(Self::ExploreNormalGiftBond),
            "EXPLORE_HATE_GIFT_BOND" => Some(Self::ExploreHateGiftBond),
            "EXPLORE_TUTORIAL_EXPLORE_POINT_MASTER_ID" => {
                Some(Self::ExploreTutorialExplorePointMasterId)
            }
            "EXPLORE_COIN_DAILY_LIMIT" => Some(Self::ExploreCoinDailyLimit),
            "EXPLORE_COIN_DAILY_LIMIT_MAX" => Some(Self::ExploreCoinDailyLimitMax),
            "TEABREAK_FAVORITE_ITEM" => Some(Self::TeabreakFavoriteItem),
            "TEABREAK_LIKE_ITEM" => Some(Self::TeabreakLikeItem),
            "TEABREAK_NORMAL_ITEM" => Some(Self::TeabreakNormalItem),
            "DRINK_MAKING_A_REQUIRE_MARK" => Some(Self::DrinkMakingARequireMark),
            "DRINK_MAKING_S_REQUIRE_MARK" => Some(Self::DrinkMakingSRequireMark),
            "DRINK_MAKING_ACCESSORIES_COUNT" => Some(Self::DrinkMakingAccessoriesCount),
            "DAILY_KEYWORD_MAX_DROP_COUNT" => Some(Self::DailyKeywordMaxDropCount),
            "DAILY_KEYWORD_GUARANTEED_COUNT" => Some(Self::DailyKeywordGuaranteedCount),
            "MAIL_BOX_CAPACITY" => Some(Self::MailBoxCapacity),
            "GIFT_CODE_MAIL_TEMPLATE_ID" => Some(Self::GiftCodeMailTemplateId),
            "RECHARGE_REBATE_TEMPLATE_ID" => Some(Self::RechargeRebateTemplateId),
            "WEB_ACTIVITY_MAIL_TEMPLATE_ID" => Some(Self::WebActivityMailTemplateId),
            "CRUSADE_RESTART_TIME" => Some(Self::CrusadeRestartTime),
            "CRUSADE_BATTLE_WIN_RECOVER_AP" => Some(Self::CrusadeBattleWinRecoverAp),
            "CRUSADE_MAX_TREASURE_CHOICE" => Some(Self::CrusadeMaxTreasureChoice),
            "CRUSADE_COIN_TRANS_PERCENT" => Some(Self::CrusadeCoinTransPercent),
            "CRUSADE_COIN_INIT_LIMIT" => Some(Self::CrusadeCoinInitLimit),
            "CRUSADE_FREE_GEM_LIMIT" => Some(Self::CrusadeFreeGemLimit),
            "CRUSADE_REWARDS_MAIL_TEMPLATED_ID" => {
                Some(Self::CrusadeRewardsMailTemplatedId)
            }
            "CRUSADE_SWEEP_MAX_TIMES" => Some(Self::CrusadeSweepMaxTimes),
            "CRUSADE_SWEEP_MIN_SECONDS" => Some(Self::CrusadeSweepMinSeconds),
            "MAX_PHONEMES" => Some(Self::MaxPhonemes),
            "MAX_PHONEME_BUFFS" => Some(Self::MaxPhonemeBuffs),
            "PHONEME_LEVEL_UP_GOLD_COST" => Some(Self::PhonemeLevelUpGoldCost),
            "PHONEME_GRADE_UP_GOLD_COST" => Some(Self::PhonemeGradeUpGoldCost),
            "PHONEME_DECOMPOSE_REFUND_PERCENTAGE" => {
                Some(Self::PhonemeDecomposeRefundPercentage)
            }
            "PHONEME_DECOMPOSE_REFUND_ITEM" => Some(Self::PhonemeDecomposeRefundItem),
            "PHONEME_LEVELUP_PERCENTAGE" => Some(Self::PhonemeLevelupPercentage),
            "PHONEME_DECOMPOSE_TYPE" => Some(Self::PhonemeDecomposeType),
            "GACHA_GEM_COST" => Some(Self::GachaGemCost),
            "BATTLE_CHARACTER_MAX_RANK" => Some(Self::BattleCharacterMaxRank),
            "HOMEPAGE_SCENE_DEFAULT" => Some(Self::HomepageSceneDefault),
            "PLAYER_MONTHLY_PASS_LIMIT" => Some(Self::PlayerMonthlyPassLimit),
            "PLAYER_POP_GIFT_QUEUE_CAPACITY" => Some(Self::PlayerPopGiftQueueCapacity),
            "APPLE_STORE_GIFT_CODE_SHOP" => Some(Self::AppleStoreGiftCodeShop),
            "APPLE_STORE_GIFT_CODE_MAIL_TEMPLATE" => {
                Some(Self::AppleStoreGiftCodeMailTemplate)
            }
            "MASTER_SECTION_DURATION" => Some(Self::MasterSectionDuration),
            "MASTER_SECTION_START_TIME" => Some(Self::MasterSectionStartTime),
            "MASTER_SECTION_START_MASTER_ID" => Some(Self::MasterSectionStartMasterId),
            "MASTER_SECTION_COOLING_DOWN_TURNS" => {
                Some(Self::MasterSectionCoolingDownTurns)
            }
            "MASTER_SECTION_POLYNOMIAL_COEFFICIENT" => {
                Some(Self::MasterSectionPolynomialCoefficient)
            }
            "MASTER_SECTION_POLYNOMIAL_BASE_COEFFICIENT" => {
                Some(Self::MasterSectionPolynomialBaseCoefficient)
            }
            _ => None,
        }
    }
}
