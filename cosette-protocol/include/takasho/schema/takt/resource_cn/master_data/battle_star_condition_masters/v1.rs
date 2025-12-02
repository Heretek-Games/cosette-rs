#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BattleStarConditionMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub battle_star_condition_master_id: i64,
    #[prost(int64, tag = "3")]
    pub type_id: i64,
    #[prost(int64, repeated, tag = "4")]
    pub param: ::prost::alloc::vec::Vec<i64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BattleStarConditionMasters {
    #[prost(message, repeated, tag = "1")]
    pub battle_star_condition_masters: ::prost::alloc::vec::Vec<
        BattleStarConditionMaster,
    >,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BattleStarSatisfiedConditionType {
    StarConditionInvalid = 0,
    WinBattle = 1,
    TurnCountLessThan = 2,
    CharacterDownLessThan = 3,
}
impl BattleStarSatisfiedConditionType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            BattleStarSatisfiedConditionType::StarConditionInvalid => {
                "StarConditionInvalid"
            }
            BattleStarSatisfiedConditionType::WinBattle => "WinBattle",
            BattleStarSatisfiedConditionType::TurnCountLessThan => "TurnCountLessThan",
            BattleStarSatisfiedConditionType::CharacterDownLessThan => {
                "CharacterDownLessThan"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "StarConditionInvalid" => Some(Self::StarConditionInvalid),
            "WinBattle" => Some(Self::WinBattle),
            "TurnCountLessThan" => Some(Self::TurnCountLessThan),
            "CharacterDownLessThan" => Some(Self::CharacterDownLessThan),
            _ => None,
        }
    }
}
