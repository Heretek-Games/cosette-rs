#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BattleEnemyMaster {
    #[prost(string, tag = "99")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "1")]
    pub battle_enemy_master_id: i64,
    #[prost(int64, tag = "2")]
    pub character_id: i64,
    #[prost(int64, tag = "4")]
    pub level_initial: i64,
    #[prost(int64, tag = "5")]
    pub level_max: i64,
    #[prost(int64, tag = "6")]
    pub hp_initial: i64,
    #[prost(int64, tag = "7")]
    pub hp_max: i64,
    #[prost(int64, tag = "8")]
    pub defend_initial: i64,
    #[prost(int64, tag = "9")]
    pub defend_max: i64,
    #[prost(int64, tag = "10")]
    pub intelligence_initial: i64,
    #[prost(int64, tag = "11")]
    pub intelligence_max: i64,
    #[prost(int64, tag = "12")]
    pub mind_initial: i64,
    #[prost(int64, tag = "13")]
    pub mind_max: i64,
    #[prost(int64, tag = "14")]
    pub speed: i64,
    #[prost(int64, tag = "15")]
    pub base_timeline: i64,
    #[prost(int64, tag = "16")]
    pub r#break: i64,
    #[prost(enumeration = "SpeciesType", tag = "17")]
    pub species_type: i32,
    #[prost(int64, tag = "18")]
    pub property_rate_fire: i64,
    #[prost(int64, tag = "19")]
    pub property_rate_ice: i64,
    #[prost(int64, tag = "20")]
    pub property_rate_wind: i64,
    #[prost(int64, tag = "21")]
    pub property_rate_thunder: i64,
    #[prost(int64, tag = "22")]
    pub property_rate_holy: i64,
    #[prost(int64, tag = "23")]
    pub property_rate_dark: i64,
    #[prost(int64, tag = "24")]
    pub poison_resist_rate_0: i64,
    #[prost(int64, tag = "25")]
    pub poison_resist_rate_1: i64,
    #[prost(int64, tag = "26")]
    pub poison_resist_rate_2: i64,
    #[prost(int64, tag = "27")]
    pub paralysis_resist_rate_0: i64,
    #[prost(int64, tag = "28")]
    pub paralysis_resist_rate_1: i64,
    #[prost(int64, tag = "29")]
    pub paralysis_resist_rate_2: i64,
    #[prost(int64, tag = "30")]
    pub sleep_resist_rate_0: i64,
    #[prost(int64, tag = "31")]
    pub sleep_resist_rate_1: i64,
    #[prost(int64, tag = "32")]
    pub sleep_resist_rate_2: i64,
    #[prost(int64, tag = "33")]
    pub enemy_command_0: i64,
    #[prost(int64, tag = "34")]
    pub enemy_command_1: i64,
    #[prost(int64, tag = "35")]
    pub enemy_command_2: i64,
    #[prost(int64, tag = "36")]
    pub enemy_command_3: i64,
    #[prost(int64, tag = "37")]
    pub enemy_command_4: i64,
    #[prost(int64, tag = "38")]
    pub base_action_id: i64,
    #[prost(int64, tag = "39")]
    pub exceptional_action_id_1: i64,
    #[prost(int64, tag = "40")]
    pub exceptional_action_id_2: i64,
    #[prost(int64, tag = "41")]
    pub exceptional_action_id_3: i64,
    #[prost(bool, tag = "42")]
    pub passive_skilled: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BattleEnemyMasters {
    #[prost(message, repeated, tag = "1")]
    pub battle_enemy_masters: ::prost::alloc::vec::Vec<BattleEnemyMaster>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SpeciesType {
    InvalidSpeciesType = 0,
    Beast = 1,
    Aquatic = 2,
    Plant = 3,
    Machine = 4,
    Humanoid = 5,
}
impl SpeciesType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SpeciesType::InvalidSpeciesType => "INVALID_SPECIES_TYPE",
            SpeciesType::Beast => "BEAST",
            SpeciesType::Aquatic => "AQUATIC",
            SpeciesType::Plant => "PLANT",
            SpeciesType::Machine => "MACHINE",
            SpeciesType::Humanoid => "HUMANOID",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INVALID_SPECIES_TYPE" => Some(Self::InvalidSpeciesType),
            "BEAST" => Some(Self::Beast),
            "AQUATIC" => Some(Self::Aquatic),
            "PLANT" => Some(Self::Plant),
            "MACHINE" => Some(Self::Machine),
            "HUMANOID" => Some(Self::Humanoid),
            _ => None,
        }
    }
}
