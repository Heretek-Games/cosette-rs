#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivateTalkSummaryTalkMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub private_talk_summary_talk_master_id: i64,
    #[prost(int64, tag = "3")]
    pub battle_character_master_id: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivateTalkSummaryTalkMasters {
    #[prost(message, repeated, tag = "1")]
    pub private_talk_summary_talk_masters: ::prost::alloc::vec::Vec<
        PrivateTalkSummaryTalkMaster,
    >,
}
