#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerCrusadeSweepBattleRecord {
    #[prost(string, tag = "1")]
    pub player_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub crusade_battle_id: i64,
    #[prost(int64, tag = "3")]
    pub crusade_type_id: i64,
    #[prost(int64, tag = "4")]
    pub least_battle_time: i64,
    #[prost(int64, tag = "5")]
    pub normal_count: i64,
    #[prost(int64, tag = "6")]
    pub last_normal_timestamp: i64,
    #[prost(int64, tag = "7")]
    pub skip_count: i64,
    #[prost(int64, tag = "8")]
    pub last_skip_timestamp: i64,
    #[prost(string, repeated, tag = "9")]
    pub db_crusade_sweep_characters: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    #[prost(sint64, tag = "91")]
    pub created_at: i64,
    #[prost(sint64, tag = "92")]
    pub updated_at: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerCrusadeHistory {
    #[prost(string, tag = "1")]
    pub player_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub crusade_type_master_id: i64,
    #[prost(int64, tag = "5")]
    pub treasure_count: i64,
    #[prost(int64, tag = "6")]
    pub coin_count: i64,
    #[prost(int64, tag = "7")]
    pub pass_count: i64,
    #[prost(int64, tag = "8")]
    pub option_character_count: i64,
    #[prost(int64, tag = "9")]
    pub move_count: i64,
    #[prost(int64, tag = "10")]
    pub finish_event_count: i64,
    #[prost(int64, tag = "11")]
    pub finish_battle_count: i64,
    #[prost(int64, tag = "12")]
    pub treasure_type_count: i64,
    #[prost(int64, repeated, tag = "13")]
    pub finished_levels: ::prost::alloc::vec::Vec<i64>,
    #[prost(sint64, tag = "91")]
    pub created_at: i64,
    #[prost(sint64, tag = "92")]
    pub updated_at: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerCrusadeSweep {
    #[prost(string, tag = "1")]
    pub player_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub crusade_type_id: i64,
    #[prost(int64, tag = "3")]
    pub crusade_battle_id: i64,
    #[prost(enumeration = "CrusadeSweepStatus", tag = "4")]
    pub status: i32,
    #[prost(sint64, tag = "5")]
    pub start_at: i64,
    #[prost(sint64, tag = "6")]
    pub end_at: i64,
    #[prost(int64, tag = "7")]
    pub times: i64,
    #[prost(int64, tag = "8")]
    pub cost_per_time: i64,
    #[prost(string, repeated, tag = "9")]
    pub db_crusade_sweep_characters: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    #[prost(sint64, tag = "91")]
    pub created_at: i64,
    #[prost(sint64, tag = "92")]
    pub updated_at: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CrusadeSweepStatus {
    Spare = 0,
    Ongoing = 1,
    Finished = 2,
}
impl CrusadeSweepStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CrusadeSweepStatus::Spare => "SPARE",
            CrusadeSweepStatus::Ongoing => "ONGOING",
            CrusadeSweepStatus::Finished => "FINISHED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SPARE" => Some(Self::Spare),
            "ONGOING" => Some(Self::Ongoing),
            "FINISHED" => Some(Self::Finished),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerCrusadeCharacterStatus {
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(bool, tag = "2")]
    pub available: bool,
    #[prost(bool, tag = "3")]
    pub has: bool,
    #[prost(int64, tag = "4")]
    pub hp: i64,
    #[prost(int64, tag = "5")]
    pub ap: i64,
    #[prost(message, repeated, tag = "6")]
    pub delta: ::prost::alloc::vec::Vec<player_crusade_character_status::DeltaEntry>,
}
/// Nested message and enum types in `PlayerCrusadeCharacterStatus`.
pub mod player_crusade_character_status {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DeltaEntry {
        #[prost(int64, tag = "1")]
        pub key: i64,
        #[prost(int64, tag = "2")]
        pub value: i64,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerExtraChoice {
    #[prost(enumeration = "PlayerExtraChoiceType", tag = "1")]
    pub r#type: i32,
    #[prost(int64, tag = "2")]
    pub id: i64,
    #[prost(enumeration = "PlayerExtraChoiceFromType", tag = "3")]
    pub from: i32,
    #[prost(int64, tag = "4")]
    pub parent_id: i64,
    #[prost(int64, tag = "5")]
    pub extra: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerCrusade {
    #[prost(string, tag = "1")]
    pub player_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub crusade_type_id: i64,
    #[prost(int64, tag = "3")]
    pub crusade_level_master_id: i64,
    #[prost(int64, tag = "4")]
    pub map_master_id: i64,
    #[prost(int64, tag = "5")]
    pub node_master_id: i64,
    #[prost(int64, tag = "6")]
    pub event_master_id: i64,
    #[prost(int64, tag = "7")]
    pub crusade_battle_id: i64,
    #[prost(int64, tag = "8")]
    pub underground_section_id: i64,
    #[prost(int64, repeated, tag = "9")]
    pub supports: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, tag = "10")]
    pub last_update_time: i64,
    #[prost(message, repeated, tag = "11")]
    pub character_status: ::prost::alloc::vec::Vec<PlayerCrusadeCharacterStatus>,
    #[prost(string, repeated, tag = "12")]
    pub db_character_status: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, repeated, tag = "14")]
    pub route_nodes: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, repeated, tag = "15")]
    pub route_events: ::prost::alloc::vec::Vec<i64>,
    #[prost(enumeration = "PlayerCrusadeStatus", tag = "16")]
    pub status: i32,
    #[prost(string, repeated, tag = "17")]
    pub resource: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, tag = "18")]
    pub max_level: i64,
    #[prost(int64, tag = "19")]
    pub remain_restart_times: i64,
    #[prost(message, repeated, tag = "20")]
    pub extra_choices: ::prost::alloc::vec::Vec<PlayerExtraChoice>,
    #[prost(string, repeated, tag = "21")]
    pub db_extra_choices: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, tag = "22")]
    pub seed: i64,
    #[prost(int64, tag = "23")]
    pub env: i64,
    #[prost(message, repeated, tag = "24")]
    pub free_events: ::prost::alloc::vec::Vec<player_crusade::FreeEventsEntry>,
    #[prost(string, tag = "25")]
    pub db_free_events: ::prost::alloc::string::String,
    #[prost(int64, repeated, tag = "26")]
    pub finished_crusade_battle_id: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, repeated, tag = "27")]
    pub unlocked_crusade_battle_id: ::prost::alloc::vec::Vec<i64>,
    #[prost(message, repeated, tag = "28")]
    pub next_events: ::prost::alloc::vec::Vec<player_crusade::NextEventsEntry>,
    #[prost(string, tag = "29")]
    pub db_next_events: ::prost::alloc::string::String,
    #[prost(sint64, tag = "30")]
    pub expired_at: i64,
    #[prost(int64, tag = "31")]
    pub receive_rewards_times: i64,
    #[prost(sint64, tag = "91")]
    pub created_at: i64,
    #[prost(sint64, tag = "92")]
    pub updated_at: i64,
}
/// Nested message and enum types in `PlayerCrusade`.
pub mod player_crusade {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FreeEventsEntry {
        #[prost(int64, tag = "1")]
        pub key: i64,
        #[prost(int64, tag = "2")]
        pub value: i64,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NextEventsEntry {
        #[prost(int64, tag = "1")]
        pub key: i64,
        #[prost(int64, tag = "2")]
        pub value: i64,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PlayerCrusadeStatus {
    InvalidPlayerCrusadeStatus = 0,
    Start = 1,
    ProgressOn = 2,
    Finish = 3,
    Settle = 4,
    PlayerCrusadeEventStart = 5,
    PlayerCrusadeOptionChosen = 6,
    PlayerCrusadeEventEnd = 7,
}
impl PlayerCrusadeStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PlayerCrusadeStatus::InvalidPlayerCrusadeStatus => {
                "InvalidPlayerCrusadeStatus"
            }
            PlayerCrusadeStatus::Start => "PlayerCrusadeStatusStart",
            PlayerCrusadeStatus::ProgressOn => "PlayerCrusadeStatusProgressOn",
            PlayerCrusadeStatus::Finish => "PlayerCrusadeStatusFinish",
            PlayerCrusadeStatus::Settle => "PlayerCrusadeStatusSettle",
            PlayerCrusadeStatus::PlayerCrusadeEventStart => "PlayerCrusadeEventStart",
            PlayerCrusadeStatus::PlayerCrusadeOptionChosen => "PlayerCrusadeOptionChosen",
            PlayerCrusadeStatus::PlayerCrusadeEventEnd => "PlayerCrusadeEventEnd",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "InvalidPlayerCrusadeStatus" => Some(Self::InvalidPlayerCrusadeStatus),
            "PlayerCrusadeStatusStart" => Some(Self::Start),
            "PlayerCrusadeStatusProgressOn" => Some(Self::ProgressOn),
            "PlayerCrusadeStatusFinish" => Some(Self::Finish),
            "PlayerCrusadeStatusSettle" => Some(Self::Settle),
            "PlayerCrusadeEventStart" => Some(Self::PlayerCrusadeEventStart),
            "PlayerCrusadeOptionChosen" => Some(Self::PlayerCrusadeOptionChosen),
            "PlayerCrusadeEventEnd" => Some(Self::PlayerCrusadeEventEnd),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PlayerExtraChoiceFromType {
    Invalid = 0,
    Option = 1,
    Event = 2,
}
impl PlayerExtraChoiceFromType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PlayerExtraChoiceFromType::Invalid => "PlayerExtraChoiceFromTypeInvalid",
            PlayerExtraChoiceFromType::Option => "PlayerExtraChoiceFromTypeOption",
            PlayerExtraChoiceFromType::Event => "PlayerExtraChoiceFromTypeEvent",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PlayerExtraChoiceFromTypeInvalid" => Some(Self::Invalid),
            "PlayerExtraChoiceFromTypeOption" => Some(Self::Option),
            "PlayerExtraChoiceFromTypeEvent" => Some(Self::Event),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PlayerExtraChoiceType {
    Invalid = 0,
    Treasure = 1,
    Character = 2,
}
impl PlayerExtraChoiceType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PlayerExtraChoiceType::Invalid => "PlayerExtraChoiceTypeInvalid",
            PlayerExtraChoiceType::Treasure => "PlayerExtraChoiceTypeTreasure",
            PlayerExtraChoiceType::Character => "PlayerExtraChoiceTypeCharacter",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PlayerExtraChoiceTypeInvalid" => Some(Self::Invalid),
            "PlayerExtraChoiceTypeTreasure" => Some(Self::Treasure),
            "PlayerExtraChoiceTypeCharacter" => Some(Self::Character),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerCrusadeRecord {
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(int64, tag = "2")]
    pub receive_at: i64,
    #[prost(int64, tag = "3")]
    pub times: i64,
    #[prost(int64, tag = "4")]
    pub durability: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerCrusadeTreasure {
    #[prost(string, tag = "1")]
    pub player_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub crusade_type_id: i64,
    #[prost(int64, repeated, tag = "3")]
    pub unlock: ::prost::alloc::vec::Vec<i64>,
    #[prost(string, repeated, tag = "4")]
    pub available: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "5")]
    pub treasures: ::prost::alloc::vec::Vec<PlayerCrusadeRecord>,
    #[prost(int64, repeated, tag = "6")]
    pub collections: ::prost::alloc::vec::Vec<i64>,
    #[prost(sint64, tag = "91")]
    pub created_at: i64,
    #[prost(sint64, tag = "92")]
    pub updated_at: i64,
}
