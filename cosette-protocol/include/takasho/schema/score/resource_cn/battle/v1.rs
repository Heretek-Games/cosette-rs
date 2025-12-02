#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BattleStartParam {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BattleStartResult {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub seed: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BattleEndParam {
    #[prost(message, optional, tag = "1")]
    pub battle_achieves: ::core::option::Option<BattleAchievement>,
    #[prost(message, repeated, tag = "2")]
    pub start: ::prost::alloc::vec::Vec<battle_end_param::Status>,
    #[prost(message, repeated, tag = "3")]
    pub end: ::prost::alloc::vec::Vec<battle_end_param::Status>,
    #[prost(int64, tag = "11")]
    pub round: i64,
    #[prost(int64, repeated, tag = "12")]
    pub dead_card: ::prost::alloc::vec::Vec<i64>,
    #[prost(message, repeated, tag = "13")]
    pub damage: ::prost::alloc::vec::Vec<battle_end_param::DamageEntry>,
    #[prost(message, repeated, tag = "14")]
    pub skill_use: ::prost::alloc::vec::Vec<battle_end_param::SkillUseEntry>,
    #[prost(bool, tag = "15")]
    pub auto: bool,
    #[prost(int64, repeated, tag = "16")]
    pub monster: ::prost::alloc::vec::Vec<i64>,
    #[prost(bool, tag = "17")]
    pub win: bool,
    #[prost(int64, repeated, tag = "18")]
    pub achievements: ::prost::alloc::vec::Vec<i64>,
}
/// Nested message and enum types in `BattleEndParam`.
pub mod battle_end_param {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DamageEntry {
        #[prost(int64, tag = "1")]
        pub key: i64,
        #[prost(int64, tag = "2")]
        pub value: i64,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SkillUseEntry {
        #[prost(int64, tag = "1")]
        pub key: i64,
        #[prost(int64, tag = "2")]
        pub value: i64,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Status {
        #[prost(int64, tag = "1")]
        pub character_id: i64,
        #[prost(int64, tag = "2")]
        pub hp: i64,
        #[prost(int64, tag = "3")]
        pub ap: i64,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BattleEndResult {
    #[prost(bool, tag = "1")]
    pub valid: bool,
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    #[prost(bool, tag = "3")]
    pub star_first_satisfied: bool,
    #[prost(bool, tag = "4")]
    pub star_second_satisfied: bool,
    #[prost(bool, tag = "5")]
    pub star_third_satisfied: bool,
    #[prost(message, repeated, tag = "6")]
    pub reward_contents: ::prost::alloc::vec::Vec<
        battle_end_result::RewardContentsEntry,
    >,
    #[prost(message, repeated, tag = "7")]
    pub first_clear_reward: ::prost::alloc::vec::Vec<
        battle_end_result::FirstClearRewardEntry,
    >,
    #[prost(message, repeated, tag = "8")]
    pub achievements_reward: ::prost::alloc::vec::Vec<
        battle_end_result::AchievementsRewardEntry,
    >,
    #[prost(bool, tag = "9")]
    pub is_first: bool,
    #[prost(int64, repeated, tag = "10")]
    pub achievements: ::prost::alloc::vec::Vec<i64>,
}
/// Nested message and enum types in `BattleEndResult`.
pub mod battle_end_result {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RewardContentsEntry {
        #[prost(int64, tag = "1")]
        pub key: i64,
        #[prost(int64, tag = "2")]
        pub value: i64,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FirstClearRewardEntry {
        #[prost(int64, tag = "1")]
        pub key: i64,
        #[prost(int64, tag = "2")]
        pub value: i64,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AchievementsRewardEntry {
        #[prost(int64, tag = "1")]
        pub key: i64,
        #[prost(int64, tag = "2")]
        pub value: i64,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BattleAchievement {
    #[prost(int64, tag = "1")]
    pub monster_killed_count: i64,
    #[prost(int64, tag = "2")]
    pub beat_off_enemy_count: i64,
    #[prost(int64, tag = "3")]
    pub pull_enemy_count: i64,
    #[prost(int64, tag = "4")]
    pub beaten_off_by_enemy_count: i64,
    #[prost(int64, tag = "5")]
    pub pulled_by_enemy_count: i64,
    #[prost(message, repeated, tag = "6")]
    pub buffed_counts: ::prost::alloc::vec::Vec<battle_achievement::BuffedCountsEntry>,
    #[prost(int64, tag = "7")]
    pub support_count: i64,
}
/// Nested message and enum types in `BattleAchievement`.
pub mod battle_achievement {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BuffedCountsEntry {
        #[prost(int64, tag = "1")]
        pub key: i64,
        #[prost(int64, tag = "2")]
        pub value: i64,
    }
}
