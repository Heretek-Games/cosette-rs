#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DailyQuestChapterMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub daily_quest_chapter_master_id: i64,
    #[prost(int64, tag = "3")]
    pub daily_quest_type_master_id: i64,
    #[prost(int64, repeated, tag = "4")]
    pub open_weekdays: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, tag = "5")]
    pub require_stage_id: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DailyQuestChapterMasters {
    #[prost(message, repeated, tag = "1")]
    pub daily_quest_chapter_masters: ::prost::alloc::vec::Vec<DailyQuestChapterMaster>,
}
