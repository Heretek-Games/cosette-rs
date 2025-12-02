#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerActivityCheckIn {
    #[prost(string, tag = "1")]
    pub player_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub activity_check_in_master_id: i64,
    #[prost(int64, repeated, tag = "3")]
    pub activity_check_in_entry_ids: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, repeated, tag = "4")]
    pub activity_check_in_entry_reward_ids: ::prost::alloc::vec::Vec<i64>,
    #[prost(sint64, tag = "81")]
    pub last_check_in_at: i64,
    #[prost(sint64, tag = "82")]
    pub limited_at: i64,
    #[prost(sint64, tag = "91")]
    pub created_at: i64,
    #[prost(sint64, tag = "92")]
    pub updated_at: i64,
}
