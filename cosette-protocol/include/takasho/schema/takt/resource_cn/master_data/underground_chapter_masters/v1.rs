#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UndergroundChapterMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub underground_chapter_master_id: i64,
    #[prost(int64, tag = "3")]
    pub underground_level_master_id: i64,
    #[prost(string, tag = "11")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "12")]
    pub unlock_condition: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "13")]
    pub start_cost_contents: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UndergroundChapterMasters {
    #[prost(message, repeated, tag = "1")]
    pub underground_chapter_masters: ::prost::alloc::vec::Vec<UndergroundChapterMaster>,
}
