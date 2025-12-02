#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BattleZoneMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub battle_zone_master_id: i64,
    #[prost(enumeration = "BattleZoneType", tag = "3")]
    pub r#type: i32,
    #[prost(int64, tag = "4")]
    pub related_id: i64,
    #[prost(bool, tag = "5")]
    pub has_star_rewards: bool,
    #[prost(int64, tag = "6")]
    pub require_star_num_1: i64,
    #[prost(string, repeated, tag = "7")]
    pub star_reward_contents_1: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, tag = "8")]
    pub require_star_num_2: i64,
    #[prost(string, repeated, tag = "9")]
    pub star_reward_contents_2: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, tag = "10")]
    pub require_star_num_3: i64,
    #[prost(string, repeated, tag = "11")]
    pub star_reward_contents_3: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BattleZoneMasters {
    #[prost(message, repeated, tag = "1")]
    pub battle_zone_masters: ::prost::alloc::vec::Vec<BattleZoneMaster>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BattleZoneType {
    InvalidBattleZoneType = 0,
    BattleZoneChapter = 1,
    BattleZonePrivateTalk = 2,
}
impl BattleZoneType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            BattleZoneType::InvalidBattleZoneType => "InvalidBattleZoneType",
            BattleZoneType::BattleZoneChapter => "BattleZoneChapter",
            BattleZoneType::BattleZonePrivateTalk => "BattleZonePrivateTalk",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "InvalidBattleZoneType" => Some(Self::InvalidBattleZoneType),
            "BattleZoneChapter" => Some(Self::BattleZoneChapter),
            "BattleZonePrivateTalk" => Some(Self::BattleZonePrivateTalk),
            _ => None,
        }
    }
}
