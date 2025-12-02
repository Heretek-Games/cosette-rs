#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChapterUnlockMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub chapter_unlock_master_id: i64,
    #[prost(string, repeated, tag = "3")]
    pub item: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChapterUnlockMasters {
    #[prost(message, repeated, tag = "1")]
    pub chapter_unlock_masters: ::prost::alloc::vec::Vec<ChapterUnlockMaster>,
}
