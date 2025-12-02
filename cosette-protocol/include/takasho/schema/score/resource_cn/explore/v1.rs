#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerExploreEvent {
    #[prost(string, tag = "1")]
    pub player_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub explore_event_master_id: i64,
    #[prost(int64, repeated, tag = "3")]
    pub unlocked_explore_adv_master_ids: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, repeated, tag = "4")]
    pub unlocked_explore_adv_master_ids_history: ::prost::alloc::vec::Vec<i64>,
    #[prost(enumeration = "Status", tag = "5")]
    pub status: i32,
    #[prost(string, repeated, tag = "6")]
    pub flags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(sint64, tag = "11")]
    pub unlocked_at: i64,
    #[prost(sint64, tag = "12")]
    pub started_at: i64,
    #[prost(sint64, tag = "13")]
    pub expired_at: i64,
    #[prost(int64, tag = "17")]
    pub used_clock: i64,
    #[prost(int64, tag = "18")]
    pub total_finished_count: i64,
    #[prost(bool, tag = "19")]
    pub liked: bool,
    #[prost(message, optional, tag = "20")]
    pub limit: ::core::option::Option<super::super::common::v1::Limit>,
    #[prost(string, tag = "21")]
    pub limit_db: ::prost::alloc::string::String,
    #[prost(bool, tag = "22")]
    pub can_start: bool,
    #[prost(int64, tag = "23")]
    pub current_battle_stage_master_id: i64,
    #[prost(sint64, tag = "91")]
    pub created_at: i64,
    #[prost(sint64, tag = "92")]
    pub updated_at: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Status {
    InvalidStatus = 0,
    Available = 1,
    Started = 2,
    Finished = 3,
}
impl Status {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Status::InvalidStatus => "INVALID_STATUS",
            Status::Available => "AVAILABLE",
            Status::Started => "STARTED",
            Status::Finished => "FINISHED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INVALID_STATUS" => Some(Self::InvalidStatus),
            "AVAILABLE" => Some(Self::Available),
            "STARTED" => Some(Self::Started),
            "FINISHED" => Some(Self::Finished),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerExploreFreeExplore {
    #[prost(string, tag = "1")]
    pub player_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub explore_free_explore_master_id: i64,
    #[prost(enumeration = "Status", tag = "3")]
    pub status: i32,
    #[prost(int64, tag = "4")]
    pub explore_object_master_id: i64,
    #[prost(int64, tag = "5")]
    pub explore_point_master_id: i64,
    #[prost(sint64, tag = "11")]
    pub started_at: i64,
    #[prost(sint64, tag = "12")]
    pub expired_at: i64,
    #[prost(int64, tag = "16")]
    pub total_finished_count: i64,
    #[prost(message, optional, tag = "20")]
    pub limit: ::core::option::Option<super::super::common::v1::Limit>,
    #[prost(string, tag = "21")]
    pub limit_db: ::prost::alloc::string::String,
    #[prost(sint64, tag = "91")]
    pub created_at: i64,
    #[prost(sint64, tag = "92")]
    pub updated_at: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerExplore {
    #[prost(string, tag = "2")]
    pub player_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "11")]
    pub energy: i64,
    #[prost(message, repeated, tag = "12")]
    pub current_battle_characters: ::prost::alloc::vec::Vec<
        PlayerExploreBattleCharacter,
    >,
    #[prost(string, repeated, tag = "13")]
    pub flags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "16")]
    pub db_current_battle_characters: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    #[prost(sint64, tag = "17")]
    pub next_battle_character_master_refresh_at: i64,
    #[prost(sint64, tag = "18")]
    pub next_refresh_at: i64,
    #[prost(message, repeated, tag = "21")]
    pub events: ::prost::alloc::vec::Vec<PlayerExploreEvent>,
    #[prost(message, repeated, tag = "22")]
    pub free_explores: ::prost::alloc::vec::Vec<PlayerExploreFreeExplore>,
    #[prost(int64, tag = "31")]
    pub homepage_show_character: i64,
    #[prost(int64, tag = "32")]
    pub homepage_show_npc: i64,
    #[prost(int64, tag = "33")]
    pub homepage_show_update_at: i64,
    #[prost(int64, tag = "34")]
    pub homepage_daily_update_at: i64,
    #[prost(int64, tag = "35")]
    pub homepage_daily_seed: i64,
    #[prost(int64, repeated, tag = "36")]
    pub daily_treasures: ::prost::alloc::vec::Vec<i64>,
    #[prost(string, repeated, tag = "37")]
    pub unlocked_scenes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, repeated, tag = "38")]
    pub bond_teabreak_characters: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, tag = "39")]
    pub get_keyword_daily_count: i64,
    #[prost(int64, tag = "40")]
    pub open_treasure_daily_count: i64,
    #[prost(sint64, tag = "91")]
    pub created_at: i64,
    #[prost(sint64, tag = "92")]
    pub updated_at: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerExploreBattleCharacter {
    #[prost(int64, tag = "1")]
    pub battle_character_master_id: i64,
    #[prost(int64, tag = "2")]
    pub explore_point_master_id: i64,
}
