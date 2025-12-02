#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExploreHomepagePointMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub explore_homepage_point_master_id: i64,
    #[prost(int64, tag = "3")]
    pub homepage_show_master_id: i64,
    #[prost(int64, tag = "4")]
    pub character_master_id: i64,
    #[prost(int64, tag = "5")]
    pub point_master_id: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExploreHomepagePointMasters {
    #[prost(message, repeated, tag = "1")]
    pub explore_homepage_point_masters: ::prost::alloc::vec::Vec<
        ExploreHomepagePointMaster,
    >,
}
