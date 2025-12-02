#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TutorialTriggerMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub tutorial_trigger_master_id: i64,
    #[prost(int64, tag = "11")]
    pub start_step: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TutorialTriggerMasters {
    #[prost(message, repeated, tag = "1")]
    pub tutorial_trigger_masters: ::prost::alloc::vec::Vec<TutorialTriggerMaster>,
}
