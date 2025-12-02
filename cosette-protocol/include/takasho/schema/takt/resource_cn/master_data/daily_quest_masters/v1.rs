#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DailyQuestMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub daily_quest_master_id: i64,
    #[prost(int64, tag = "3")]
    pub chapter_master_id: i64,
    #[prost(int64, tag = "4")]
    pub pre_daily_quest_master_id: i64,
    #[prost(int64, tag = "5")]
    pub require_player_level: i64,
    #[prost(int64, tag = "6")]
    pub use_stamina: i64,
    #[prost(int64, tag = "7")]
    pub exp_player: i64,
    #[prost(int64, tag = "8")]
    pub battle_master_id: i64,
    #[prost(int64, tag = "9")]
    pub exp_battle_character: i64,
    #[prost(int64, tag = "10")]
    pub game_money: i64,
    #[prost(string, repeated, tag = "11")]
    pub first_clear_reward_contents: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    #[prost(int64, repeated, tag = "12")]
    pub drop_master_ids: ::prost::alloc::vec::Vec<i64>,
    #[prost(string, repeated, tag = "13")]
    pub skip_reward_contents: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, repeated, tag = "14")]
    pub bonus_drop_master_ids: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, repeated, tag = "15")]
    pub display_drop_reward_contents: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, tag = "16")]
    pub daily_quest_type: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DailyQuestMasters {
    #[prost(message, repeated, tag = "1")]
    pub daily_quest_masters: ::prost::alloc::vec::Vec<DailyQuestMaster>,
}
