#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerBattleCharacter {
    #[prost(string, tag = "1")]
    pub player_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    pub battle_character_master_id: i64,
    #[prost(
        enumeration = "super::super::super::super::takt::resource_cn::master_data::battle_character_masters::v1::Rarity",
        tag = "12"
    )]
    pub rarity: i32,
    #[prost(int64, tag = "13")]
    pub exp: i64,
    #[prost(int64, tag = "14")]
    pub rank: i64,
    #[prost(int64, repeated, tag = "15")]
    pub activated_sub_rank_ids: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, repeated, tag = "16")]
    pub activated_character_panel_master_ids: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, tag = "17")]
    pub grade: i64,
    #[prost(int64, tag = "18")]
    pub engaged: i64,
    #[prost(int64, repeated, tag = "21")]
    pub skill_ids: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, repeated, tag = "22")]
    pub skill_slot_status: ::prost::alloc::vec::Vec<i64>,
    #[prost(sint64, tag = "91")]
    pub created_at: i64,
    #[prost(sint64, tag = "92")]
    pub updated_at: i64,
    #[prost(int64, repeated, tag = "70")]
    pub suits: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, repeated, tag = "71")]
    pub suit_buff_ids: ::prost::alloc::vec::Vec<i64>,
}
/// Nested message and enum types in `PlayerBattleCharacter`.
pub mod player_battle_character {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum SkillSlotStatus {
        LockedSlot = 0,
        UnlockedSlot = 1,
    }
    impl SkillSlotStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SkillSlotStatus::LockedSlot => "LOCKED_SLOT",
                SkillSlotStatus::UnlockedSlot => "UNLOCKED_SLOT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "LOCKED_SLOT" => Some(Self::LockedSlot),
                "UNLOCKED_SLOT" => Some(Self::UnlockedSlot),
                _ => None,
            }
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerBattleCharacterBond {
    #[prost(string, tag = "1")]
    pub player_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    pub battle_character_master_id: i64,
    #[prost(int64, tag = "11")]
    pub bonds: i64,
    #[prost(int64, repeated, tag = "12")]
    pub bond_achievement_master_ids: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, tag = "13")]
    pub talk_level: i64,
    #[prost(sint64, tag = "91")]
    pub created_at: i64,
    #[prost(sint64, tag = "92")]
    pub updated_at: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerBattleCharacterHistory {
    #[prost(string, tag = "1")]
    pub player_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub battle_character_master_id: i64,
    #[prost(int64, tag = "11")]
    pub battle_win_count: i64,
    #[prost(int64, tag = "12")]
    pub private_talk_count: i64,
    #[prost(string, tag = "13")]
    pub finished_explore_puzzle_count: ::prost::alloc::string::String,
    #[prost(int64, repeated, tag = "14")]
    pub finished_explore_puzzle_set: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, tag = "15")]
    pub finished_explore_invite_event_count: i64,
    #[prost(int64, repeated, tag = "22")]
    pub unlocked_bonds_infos: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, repeated, tag = "16")]
    pub finished_explore_event_set: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, tag = "17")]
    pub killed_count_in_battle: i64,
    #[prost(sint64, tag = "91")]
    pub created_at: i64,
    #[prost(sint64, tag = "92")]
    pub updated_at: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerBattleCharacterBondsAttribute {
    #[prost(string, tag = "1")]
    pub player_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub bonds_attribute_master_id: i64,
    #[prost(int64, tag = "11")]
    pub bonds_attribute_plate_level: i64,
    #[prost(sint64, tag = "91")]
    pub created_at: i64,
    #[prost(sint64, tag = "92")]
    pub updated_at: i64,
}
