#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DailyQuestTypeMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub daily_quest_type_master_id: i64,
    #[prost(int64, tag = "3")]
    pub drop_reward_times: i64,
    #[prost(int64, repeated, tag = "4")]
    pub display_drop_reward_contents: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, tag = "5")]
    pub require_stage_id: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DailyQuestTypeMasters {
    #[prost(message, repeated, tag = "1")]
    pub daily_quest_type_masters: ::prost::alloc::vec::Vec<DailyQuestTypeMaster>,
}
