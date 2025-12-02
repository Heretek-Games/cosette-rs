#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChapterMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub chapter_master_id: i64,
    #[prost(int64, tag = "3")]
    pub chapter: i64,
    #[prost(int64, tag = "4")]
    pub section: i64,
    #[prost(int64, tag = "5")]
    pub pre_id: i64,
    #[prost(int64, tag = "6")]
    pub require_player_level: i64,
    #[prost(string, repeated, tag = "62")]
    pub unlock_condition: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, tag = "61")]
    pub require_memory_quest_master_id: i64,
    #[prost(enumeration = "SectionType", tag = "7")]
    pub section_type: i32,
    #[prost(int64, tag = "12")]
    pub battle_zone_id: i64,
    #[prost(string, repeated, tag = "8")]
    pub first_clear_rewards: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, tag = "9")]
    pub adv_battle_id: i64,
    #[prost(string, tag = "10")]
    pub scenario_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "11")]
    pub character_master_id: i64,
    #[prost(int64, tag = "21")]
    pub consume_stamina: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChapterMasters {
    #[prost(message, repeated, tag = "1")]
    pub chapter_masters: ::prost::alloc::vec::Vec<ChapterMaster>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SectionType {
    Invalid = 0,
    Search = 1,
    Battle = 2,
    Inter = 3,
    Adv = 4,
}
impl SectionType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SectionType::Invalid => "INVALID",
            SectionType::Search => "SEARCH",
            SectionType::Battle => "BATTLE",
            SectionType::Inter => "INTER",
            SectionType::Adv => "ADV",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INVALID" => Some(Self::Invalid),
            "SEARCH" => Some(Self::Search),
            "BATTLE" => Some(Self::Battle),
            "INTER" => Some(Self::Inter),
            "ADV" => Some(Self::Adv),
            _ => None,
        }
    }
}
