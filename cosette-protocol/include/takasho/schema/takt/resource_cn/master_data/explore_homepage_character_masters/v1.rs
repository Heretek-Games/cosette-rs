#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExploreHomepageCharacterMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub explore_homepage_character_master_id: i64,
    #[prost(int64, tag = "3")]
    pub character_master_id: i64,
    #[prost(string, repeated, tag = "4")]
    pub unlock_conditions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExploreHomepageCharacterMasters {
    #[prost(message, repeated, tag = "1")]
    pub explore_homepage_character_masters: ::prost::alloc::vec::Vec<
        ExploreHomepageCharacterMaster,
    >,
}
