#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerMemoryQuest {
    #[prost(string, tag = "1")]
    pub player_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub memory_quest_master_id: i64,
    #[prost(bool, tag = "4")]
    pub star1: bool,
    #[prost(bool, tag = "5")]
    pub star2: bool,
    #[prost(bool, tag = "6")]
    pub star3: bool,
    #[prost(sint64, tag = "11")]
    pub last_challenge_time: i64,
    #[prost(int64, tag = "12")]
    pub challenged_count: i64,
    #[prost(sint64, tag = "91")]
    pub created_at: i64,
    #[prost(sint64, tag = "92")]
    pub updated_at: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerMemoryQuestChapter {
    #[prost(string, tag = "1")]
    pub player_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub memory_quest_chapter_master_id: i64,
    #[prost(sint64, tag = "3")]
    pub received_star_reward_contents_1_time: i64,
    #[prost(sint64, tag = "4")]
    pub received_star_reward_contents_2_time: i64,
    #[prost(sint64, tag = "5")]
    pub received_star_reward_contents_3_time: i64,
    #[prost(sint64, tag = "91")]
    pub created_at: i64,
    #[prost(sint64, tag = "92")]
    pub updated_at: i64,
}
