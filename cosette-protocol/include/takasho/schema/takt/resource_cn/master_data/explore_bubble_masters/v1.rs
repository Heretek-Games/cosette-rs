#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExploreBubbleMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub explore_bubble_master_id: i64,
    #[prost(int64, tag = "3")]
    pub mini_character_id: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExploreBubbleMasters {
    #[prost(message, repeated, tag = "1")]
    pub explore_bubble_masters: ::prost::alloc::vec::Vec<ExploreBubbleMaster>,
}
