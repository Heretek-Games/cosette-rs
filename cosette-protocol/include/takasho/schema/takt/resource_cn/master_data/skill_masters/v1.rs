#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SkillMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub skill_master_id: i64,
    #[prost(int64, tag = "3")]
    pub skill_type: i64,
    #[prost(int64, tag = "4")]
    pub group_id: i64,
    #[prost(string, tag = "5")]
    pub name: ::prost::alloc::string::String,
    #[prost(int64, tag = "6")]
    pub level: i64,
    #[prost(string, tag = "7")]
    pub icon_path: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub time_line_name: ::prost::alloc::string::String,
    #[prost(int64, tag = "9")]
    pub effect_tabled_id: i64,
    #[prost(string, tag = "10")]
    pub intro_message: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub intro_time_line_name: ::prost::alloc::string::String,
    #[prost(string, tag = "12")]
    pub musical_effect_bg: ::prost::alloc::string::String,
    #[prost(int64, tag = "13")]
    pub musical_effect_turn: i64,
    #[prost(string, tag = "14")]
    pub bgm_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "15")]
    pub cuntin_message_id: i64,
    #[prost(string, tag = "16")]
    pub cuntin_path: ::prost::alloc::string::String,
    #[prost(int64, tag = "17")]
    pub strategy_type_id: i64,
    #[prost(int64, tag = "18")]
    pub target_type: i64,
    #[prost(bool, repeated, tag = "19")]
    pub self_position: ::prost::alloc::vec::Vec<bool>,
    #[prost(bool, repeated, tag = "20")]
    pub target_position: ::prost::alloc::vec::Vec<bool>,
    #[prost(int64, tag = "21")]
    pub damage_type: i64,
    #[prost(int64, tag = "22")]
    pub cd: i64,
    #[prost(int64, repeated, tag = "23")]
    pub energy_decrease: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, repeated, tag = "24")]
    pub energy_increase: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, tag = "25")]
    pub time_scale: i64,
    #[prost(int64, tag = "26")]
    pub tempo_milli_seconds: i64,
    #[prost(int64, tag = "27")]
    pub hit_count: i64,
    #[prost(int64, repeated, tag = "28")]
    pub hit_rate: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, repeated, tag = "29")]
    pub hit_delay: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, repeated, tag = "30")]
    pub hit_effect_id: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, repeated, tag = "31")]
    pub all_hit_effect_id: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, repeated, tag = "32")]
    pub unique_hit_effect_id: ::prost::alloc::vec::Vec<i64>,
    #[prost(bool, tag = "33")]
    pub enable_counter: bool,
    #[prost(bool, tag = "34")]
    pub enable_dual_guard: bool,
    #[prost(bool, tag = "35")]
    pub enable_chain_attack: bool,
    #[prost(int64, repeated, tag = "36")]
    pub passive_buff: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, repeated, tag = "361")]
    pub passive_buff_stock: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, repeated, tag = "37")]
    pub create_buff: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, repeated, tag = "371")]
    pub create_buff_stock: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, repeated, tag = "38")]
    pub impact: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, repeated, tag = "39")]
    pub shake_id: ::prost::alloc::vec::Vec<i64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SkillMasters {
    #[prost(message, repeated, tag = "1")]
    pub skill_masters: ::prost::alloc::vec::Vec<SkillMaster>,
}
