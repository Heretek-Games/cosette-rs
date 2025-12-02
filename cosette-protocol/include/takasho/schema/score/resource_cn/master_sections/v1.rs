#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerMasterSection {
    #[prost(string, tag = "1")]
    pub player_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub master_section_master_id: i64,
    #[prost(int64, tag = "3")]
    pub arrived_time: i64,
    #[prost(string, tag = "4")]
    pub db_chose_effects: ::prost::alloc::string::String,
    #[prost(int64, repeated, tag = "5")]
    pub chose_side_effects: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, tag = "6")]
    pub arrived_level: i64,
    #[prost(int64, repeated, tag = "7")]
    pub arrived_deck: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, tag = "8")]
    pub arrived_turns: i64,
    #[prost(message, repeated, tag = "9")]
    pub chose_effects: ::prost::alloc::vec::Vec<PlayerMasterSectionEffectStatus>,
    #[prost(int64, tag = "10")]
    pub last_update_at: i64,
    #[prost(int64, repeated, tag = "11")]
    pub now_effects: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, repeated, tag = "12")]
    pub now_side_effects: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, tag = "13")]
    pub points: i64,
    #[prost(string, tag = "14")]
    pub battle_key: ::prost::alloc::string::String,
    #[prost(sint64, tag = "91")]
    pub created_at: i64,
    #[prost(sint64, tag = "92")]
    pub updated_at: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerMasterSectionEffectStatus {
    #[prost(int64, tag = "1")]
    pub effect_master_id: i64,
    #[prost(int64, tag = "2")]
    pub cd: i64,
}
