#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerMission {
    #[prost(string, tag = "1")]
    pub player_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub mission_master_id: i64,
    #[prost(sint64, tag = "11")]
    pub receive_rewards_at: i64,
    #[prost(int64, tag = "12")]
    pub progress: i64,
    #[prost(bool, tag = "13")]
    pub can_receive: bool,
    #[prost(sint64, tag = "91")]
    pub created_at: i64,
    #[prost(sint64, tag = "92")]
    pub updated_at: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerMissionProgress {
    #[prost(string, tag = "1")]
    pub player_id: ::prost::alloc::string::String,
    #[prost(
        enumeration = "super::super::super::super::takt::resource_cn::master_data::mission_masters::v1::MissionCategory",
        tag = "2"
    )]
    pub mission_category: i32,
    #[prost(int64, tag = "11")]
    pub battle_win_count: i64,
    #[prost(int64, tag = "12")]
    pub memory_quest_play_through_count: i64,
    #[prost(int64, tag = "13")]
    pub daily_quest_play_through_count: i64,
    #[prost(int64, tag = "14")]
    pub panel_unlock_count: i64,
    #[prost(int64, tag = "15")]
    pub equipment_level_up_count: i64,
    #[prost(int64, tag = "16")]
    pub stamina_spent: i64,
    #[prost(int64, tag = "17")]
    pub battle_character_level_up_count: i64,
    #[prost(int64, tag = "18")]
    pub explore_event_finish_count: i64,
    #[prost(string, tag = "19")]
    pub sending_gift_count: ::prost::alloc::string::String,
    #[prost(int64, tag = "20")]
    pub puzzle_play_through_count: i64,
    #[prost(int64, tag = "21")]
    pub private_talk_count: i64,
    #[prost(int64, tag = "22")]
    pub purchase_stamina_with_gem_count: i64,
    #[prost(int64, tag = "23")]
    pub puzzle_drink_making_count: i64,
    #[prost(string, tag = "24")]
    pub characters_gift_count: ::prost::alloc::string::String,
    #[prost(string, tag = "26")]
    pub draw_card_with_characters_count: ::prost::alloc::string::String,
    #[prost(sint64, tag = "81")]
    pub latest_update_at: i64,
    #[prost(sint64, tag = "91")]
    pub created_at: i64,
    #[prost(sint64, tag = "92")]
    pub updated_at: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerMissionPoint {
    #[prost(string, tag = "1")]
    pub player_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub category1_mission_points: i64,
    #[prost(int64, tag = "3")]
    pub category2_mission_points: i64,
    #[prost(int64, tag = "4")]
    pub category3_mission_points: i64,
    #[prost(int64, repeated, tag = "5")]
    pub category1_received_mission_point_reward_master_ids: ::prost::alloc::vec::Vec<
        i64,
    >,
    #[prost(int64, repeated, tag = "6")]
    pub category2_received_mission_point_reward_master_ids: ::prost::alloc::vec::Vec<
        i64,
    >,
    #[prost(int64, repeated, tag = "7")]
    pub category3_received_mission_point_reward_master_ids: ::prost::alloc::vec::Vec<
        i64,
    >,
    #[prost(sint64, tag = "11")]
    pub category1_refresh_at: i64,
    #[prost(sint64, tag = "12")]
    pub category2_refresh_at: i64,
    #[prost(sint64, tag = "91")]
    pub created_at: i64,
    #[prost(sint64, tag = "92")]
    pub updated_at: i64,
}
