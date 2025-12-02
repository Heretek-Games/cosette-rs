#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerBattleZoneBattle {
    #[prost(string, tag = "1")]
    pub player_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub battle_zone_battle_master_id: i64,
    #[prost(int64, tag = "3")]
    pub star_num: i64,
    #[prost(sint64, tag = "91")]
    pub created_at: i64,
    #[prost(sint64, tag = "92")]
    pub updated_at: i64,
    #[prost(bool, tag = "101")]
    pub star_first_satisfied: bool,
    #[prost(bool, tag = "102")]
    pub star_second_satisfied: bool,
    #[prost(bool, tag = "103")]
    pub star_third_satisfied: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerBattleZone {
    #[prost(string, tag = "1")]
    pub player_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub battle_zone_master_id: i64,
    #[prost(enumeration = "PlayerBattleZoneStatus", tag = "3")]
    pub status: i32,
    #[prost(int64, tag = "4")]
    pub stage_one_update_time: i64,
    #[prost(int64, tag = "5")]
    pub stage_two_update_time: i64,
    #[prost(int64, tag = "6")]
    pub stage_three_update_time: i64,
    #[prost(int64, repeated, tag = "7")]
    pub battle_zone_chests: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, tag = "8")]
    pub star_count: i64,
    #[prost(sint64, tag = "91")]
    pub created_at: i64,
    #[prost(sint64, tag = "92")]
    pub updated_at: i64,
    #[prost(message, repeated, tag = "100")]
    pub battles: ::prost::alloc::vec::Vec<PlayerBattleZoneBattle>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PlayerBattleZoneStatus {
    Invalid = 0,
    Arrived = 1,
    Stay = 2,
    Clearable = 3,
}
impl PlayerBattleZoneStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PlayerBattleZoneStatus::Invalid => "PlayerBattleZoneStatusInvalid",
            PlayerBattleZoneStatus::Arrived => "PlayerBattleZoneStatusArrived",
            PlayerBattleZoneStatus::Stay => "PlayerBattleZoneStatusStay",
            PlayerBattleZoneStatus::Clearable => "PlayerBattleZoneStatusClearable",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PlayerBattleZoneStatusInvalid" => Some(Self::Invalid),
            "PlayerBattleZoneStatusArrived" => Some(Self::Arrived),
            "PlayerBattleZoneStatusStay" => Some(Self::Stay),
            "PlayerBattleZoneStatusClearable" => Some(Self::Clearable),
            _ => None,
        }
    }
}
