#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EquipmentMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub equipment_master_id: i64,
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    #[prost(int64, tag = "11")]
    pub rarity: i64,
    #[prost(int64, tag = "12")]
    pub initial_grade: i64,
    #[prost(int64, tag = "13")]
    pub initial_rank: i64,
    #[prost(int64, tag = "14")]
    pub exp_supply: i64,
    #[prost(enumeration = "Position", tag = "15")]
    pub position: i32,
    #[prost(int64, repeated, tag = "16")]
    pub equipable_jobs: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, tag = "20")]
    pub initial_hp: i64,
    #[prost(int64, tag = "21")]
    pub max_hp: i64,
    #[prost(int64, tag = "22")]
    pub initial_atk: i64,
    #[prost(int64, tag = "23")]
    pub max_atk: i64,
    #[prost(int64, tag = "24")]
    pub initial_def: i64,
    #[prost(int64, tag = "25")]
    pub max_def: i64,
    #[prost(int64, tag = "26")]
    pub initial_int: i64,
    #[prost(int64, tag = "27")]
    pub max_int: i64,
    #[prost(int64, tag = "28")]
    pub initial_res: i64,
    #[prost(int64, tag = "29")]
    pub max_res: i64,
    #[prost(int64, tag = "30")]
    pub initial_dex: i64,
    #[prost(int64, tag = "31")]
    pub max_dex: i64,
    #[prost(int64, tag = "32")]
    pub initial_crt: i64,
    #[prost(int64, tag = "33")]
    pub max_crt: i64,
    #[prost(int64, tag = "34")]
    pub initial_crd: i64,
    #[prost(int64, tag = "35")]
    pub max_crd: i64,
    #[prost(int64, tag = "36")]
    pub initial_acrt: i64,
    #[prost(int64, tag = "37")]
    pub max_acrt: i64,
    #[prost(int64, tag = "38")]
    pub initial_crdr: i64,
    #[prost(int64, tag = "39")]
    pub max_crdr: i64,
    #[prost(int64, tag = "41")]
    pub skill_id: i64,
    #[prost(int64, repeated, tag = "42")]
    pub skill_available_character_master_id: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, tag = "43")]
    pub max_level: i64,
    #[prost(bool, tag = "50")]
    pub is_unchanged: bool,
    #[prost(string, repeated, tag = "51")]
    pub decompose_content: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EquipmentMasters {
    #[prost(message, repeated, tag = "1")]
    pub equipment_masters: ::prost::alloc::vec::Vec<EquipmentMaster>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EquipmentRarity {
    InvalidEquipmentRarity = 0,
    N = 1,
    R = 2,
    Sr = 3,
    Ssr = 4,
}
impl EquipmentRarity {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EquipmentRarity::InvalidEquipmentRarity => "INVALID_EQUIPMENT_RARITY",
            EquipmentRarity::N => "N",
            EquipmentRarity::R => "R",
            EquipmentRarity::Sr => "SR",
            EquipmentRarity::Ssr => "SSR",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INVALID_EQUIPMENT_RARITY" => Some(Self::InvalidEquipmentRarity),
            "N" => Some(Self::N),
            "R" => Some(Self::R),
            "SR" => Some(Self::Sr),
            "SSR" => Some(Self::Ssr),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Position {
    InvalidEquipmentPosition = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
}
impl Position {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Position::InvalidEquipmentPosition => "INVALID_EQUIPMENT_POSITION",
            Position::One => "ONE",
            Position::Two => "TWO",
            Position::Three => "THREE",
            Position::Four => "FOUR",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INVALID_EQUIPMENT_POSITION" => Some(Self::InvalidEquipmentPosition),
            "ONE" => Some(Self::One),
            "TWO" => Some(Self::Two),
            "THREE" => Some(Self::Three),
            "FOUR" => Some(Self::Four),
            _ => None,
        }
    }
}
