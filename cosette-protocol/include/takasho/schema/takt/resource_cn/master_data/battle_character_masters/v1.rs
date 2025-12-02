#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BattleCharacterMaster {
    #[prost(string, tag = "99")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "1")]
    pub battle_character_master_id: i64,
    #[prost(int64, tag = "2")]
    pub character_master_id: i64,
    #[prost(int64, tag = "3")]
    pub character_type_id: i64,
    #[prost(int64, tag = "4")]
    pub sort_id: i64,
    #[prost(string, tag = "5")]
    pub name: ::prost::alloc::string::String,
    #[prost(enumeration = "Rarity", tag = "11")]
    pub rarity: i32,
    #[prost(int64, tag = "12")]
    pub initial_rank: i64,
    #[prost(int64, repeated, tag = "13")]
    pub camps: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, tag = "14")]
    pub job: i64,
    #[prost(string, repeated, tag = "15")]
    pub notes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, tag = "16")]
    pub initial_lv: i64,
    #[prost(int64, tag = "17")]
    pub initial_grade: i64,
    #[prost(int64, repeated, tag = "18")]
    pub favorite_bond_items: ::prost::alloc::vec::Vec<i64>,
    #[prost(string, repeated, tag = "19")]
    pub unlock_reward_contents: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "20")]
    pub get_rewards: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, tag = "21")]
    pub initial_hp: i64,
    #[prost(int64, tag = "22")]
    pub initial_atk: i64,
    #[prost(int64, tag = "23")]
    pub initial_def: i64,
    #[prost(int64, tag = "24")]
    pub initial_dex: i64,
    #[prost(int64, tag = "25")]
    pub initial_int: i64,
    #[prost(int64, tag = "26")]
    pub initial_res: i64,
    #[prost(int64, tag = "27")]
    pub initial_crt: i64,
    #[prost(int64, tag = "28")]
    pub initial_crd: i64,
    #[prost(int64, tag = "29")]
    pub initial_acrt: i64,
    #[prost(int64, tag = "30")]
    pub initial_crdr: i64,
    #[prost(int64, tag = "31")]
    pub hp_growth_value: i64,
    #[prost(int64, tag = "32")]
    pub atk_growth_value: i64,
    #[prost(int64, tag = "33")]
    pub def_growth_value: i64,
    #[prost(int64, tag = "34")]
    pub dex_growth_value: i64,
    #[prost(int64, tag = "35")]
    pub int_growth_value: i64,
    #[prost(int64, tag = "36")]
    pub res_growth_value: i64,
    #[prost(int64, tag = "37")]
    pub crt_growth_value: i64,
    #[prost(int64, tag = "38")]
    pub crd_growth_value: i64,
    #[prost(int64, tag = "39")]
    pub acrt_growth_value: i64,
    #[prost(int64, tag = "40")]
    pub crdr_growth_value: i64,
    #[prost(int64, tag = "41")]
    pub skill_1_id: i64,
    #[prost(int64, tag = "42")]
    pub skill_2_id: i64,
    #[prost(int64, tag = "43")]
    pub skill_3_id: i64,
    #[prost(int64, tag = "44")]
    pub musical_effect_id: i64,
    #[prost(int64, tag = "45")]
    pub passive_skill_1_id: i64,
    #[prost(int64, tag = "46")]
    pub passive_skill_2_id: i64,
    #[prost(int64, tag = "47")]
    pub counter_command_id: i64,
    #[prost(int64, tag = "48")]
    pub chain_attack_command_id: i64,
    #[prost(int64, tag = "49")]
    pub follow_attack_command_id: i64,
    #[prost(int64, repeated, tag = "50")]
    pub optional_command_ids: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, tag = "51")]
    pub initial_ap: i64,
    #[prost(int64, tag = "52")]
    pub after_contract_ap: i64,
    #[prost(int64, repeated, tag = "53")]
    pub character_tags: ::prost::alloc::vec::Vec<i64>,
    #[prost(bool, tag = "60")]
    pub is_open: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BattleCharacterMasters {
    #[prost(message, repeated, tag = "1")]
    pub battle_character_masters: ::prost::alloc::vec::Vec<BattleCharacterMaster>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Rarity {
    InvalidRarity = 0,
    N = 1,
    R = 2,
    Sr = 3,
    Ssr = 4,
}
impl Rarity {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Rarity::InvalidRarity => "INVALID_RARITY",
            Rarity::N => "N",
            Rarity::R => "R",
            Rarity::Sr => "SR",
            Rarity::Ssr => "SSR",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INVALID_RARITY" => Some(Self::InvalidRarity),
            "N" => Some(Self::N),
            "R" => Some(Self::R),
            "SR" => Some(Self::Sr),
            "SSR" => Some(Self::Ssr),
            _ => None,
        }
    }
}
