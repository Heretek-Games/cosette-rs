#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MemoryQuestChapterMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub memory_quest_chapter_master_id: i64,
    #[prost(int64, tag = "3")]
    pub chapter: i64,
    #[prost(
        enumeration = "super::super::memory_quest_masters::v1::MemoryQuestType",
        tag = "4"
    )]
    pub memory_quest_type: i32,
    #[prost(int64, tag = "11")]
    pub require_star_num_1: i64,
    #[prost(string, repeated, tag = "12")]
    pub star_reward_contents_1: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, tag = "13")]
    pub require_star_num_2: i64,
    #[prost(string, repeated, tag = "14")]
    pub star_reward_contents_2: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, tag = "15")]
    pub require_star_num_3: i64,
    #[prost(string, repeated, tag = "16")]
    pub star_reward_contents_3: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, tag = "21")]
    pub require_star_num: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MemoryQuestChapterMasters {
    #[prost(message, repeated, tag = "1")]
    pub memory_quest_chapter_masters: ::prost::alloc::vec::Vec<MemoryQuestChapterMaster>,
}
