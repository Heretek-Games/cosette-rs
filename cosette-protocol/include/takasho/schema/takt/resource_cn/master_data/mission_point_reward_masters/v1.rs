#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MissionPointRewardMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub mission_point_reward_master_id: i64,
    #[prost(
        enumeration = "super::super::mission_masters::v1::MissionCategory",
        tag = "3"
    )]
    pub mission_category: i32,
    #[prost(int64, tag = "4")]
    pub target_point: i64,
    #[prost(string, repeated, tag = "5")]
    pub rewards: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, tag = "6")]
    pub badge_master_id: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MissionPointRewardMasters {
    #[prost(message, repeated, tag = "1")]
    pub mission_point_reward_masters: ::prost::alloc::vec::Vec<MissionPointRewardMaster>,
}
