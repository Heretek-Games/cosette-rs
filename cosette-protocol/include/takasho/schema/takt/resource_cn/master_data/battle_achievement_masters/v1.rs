#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BattleAchievementMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub battle_achievement_master_id: i64,
    #[prost(enumeration = "BattleAchievementTypeEnum", tag = "3")]
    pub battle_achievement_type_ids: i32,
    #[prost(string, repeated, tag = "4")]
    pub rewards: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BattleAchievementMasters {
    #[prost(message, repeated, tag = "1")]
    pub battle_achievement_masters: ::prost::alloc::vec::Vec<BattleAchievementMaster>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BattleAchievementTypeEnum {
    Mix = 0,
    BuffAttach = 1,
    CharacterPosition = 2,
    SkillPick = 3,
    CharacterPick = 4,
    DotAttach = 5,
    EnemyOrder = 6,
    UltraComboTimes = 7,
    CampPick = 8,
}
impl BattleAchievementTypeEnum {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            BattleAchievementTypeEnum::Mix => "Mix",
            BattleAchievementTypeEnum::BuffAttach => "BuffAttach",
            BattleAchievementTypeEnum::CharacterPosition => "CharacterPosition",
            BattleAchievementTypeEnum::SkillPick => "SkillPick",
            BattleAchievementTypeEnum::CharacterPick => "CharacterPick",
            BattleAchievementTypeEnum::DotAttach => "DotAttach",
            BattleAchievementTypeEnum::EnemyOrder => "EnemyOrder",
            BattleAchievementTypeEnum::UltraComboTimes => "UltraComboTimes",
            BattleAchievementTypeEnum::CampPick => "CampPick",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Mix" => Some(Self::Mix),
            "BuffAttach" => Some(Self::BuffAttach),
            "CharacterPosition" => Some(Self::CharacterPosition),
            "SkillPick" => Some(Self::SkillPick),
            "CharacterPick" => Some(Self::CharacterPick),
            "DotAttach" => Some(Self::DotAttach),
            "EnemyOrder" => Some(Self::EnemyOrder),
            "UltraComboTimes" => Some(Self::UltraComboTimes),
            "CampPick" => Some(Self::CampPick),
            _ => None,
        }
    }
}
