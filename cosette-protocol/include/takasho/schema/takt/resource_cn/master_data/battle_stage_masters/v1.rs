#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BattleStageMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub battle_stage_master_id: i64,
    #[prost(int64, tag = "3")]
    pub battle_type: i64,
    #[prost(string, tag = "4")]
    pub background_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub start_camera_id: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub bgm_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "7")]
    pub turn_max: i64,
    #[prost(int64, tag = "8")]
    pub icon_enemy_id: i64,
    #[prost(int64, repeated, tag = "9")]
    pub enemy_setting: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, repeated, tag = "10")]
    pub enemy_ai_setting: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, repeated, tag = "11")]
    pub reinforce_setting: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, repeated, tag = "12")]
    pub reinforce_ai_setting: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, tag = "13")]
    pub victory_condition_type: i64,
    #[prost(int64, repeated, tag = "14")]
    pub victory_condition_parm: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, tag = "15")]
    pub defeat_condition_type: i64,
    #[prost(int64, repeated, tag = "16")]
    pub defeats_condition_parm: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, tag = "17")]
    pub before_battle_story: i64,
    #[prost(int64, tag = "18")]
    pub after_battle_story: i64,
    #[prost(int64, repeated, tag = "19")]
    pub force_on_char: ::prost::alloc::vec::Vec<i64>,
    #[prost(string, repeated, tag = "20")]
    pub battle_drop: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, tag = "21")]
    pub skip_prepare: i64,
    #[prost(int64, tag = "22")]
    pub enable_auto: i64,
    #[prost(int64, tag = "23")]
    pub enable_switch: i64,
    #[prost(int64, repeated, tag = "24")]
    pub locked_position: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, repeated, tag = "25")]
    pub battle_event_id: ::prost::alloc::vec::Vec<i64>,
    #[prost(bool, tag = "26")]
    pub check_star_num: bool,
    #[prost(int64, tag = "27")]
    pub star_1_condition: i64,
    #[prost(int64, tag = "29")]
    pub star_2_condition: i64,
    #[prost(int64, tag = "31")]
    pub star_3_condition: i64,
    #[prost(int64, repeated, tag = "32")]
    pub related_achievements: ::prost::alloc::vec::Vec<i64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BattleStageMasters {
    #[prost(message, repeated, tag = "1")]
    pub battle_stage_masters: ::prost::alloc::vec::Vec<BattleStageMaster>,
}
