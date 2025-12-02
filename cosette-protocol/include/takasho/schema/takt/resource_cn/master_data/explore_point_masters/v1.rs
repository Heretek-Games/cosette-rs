#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExplorePointMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub explore_point_master_id: i64,
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub map_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "11")]
    pub explore_scene_master_id: i64,
    #[prost(int64, repeated, tag = "12")]
    pub battle_character_master_ids: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, repeated, tag = "13")]
    pub explore_object_master_ids: ::prost::alloc::vec::Vec<i64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExplorePointMasters {
    #[prost(message, repeated, tag = "1")]
    pub explore_point_masters: ::prost::alloc::vec::Vec<ExplorePointMaster>,
}
