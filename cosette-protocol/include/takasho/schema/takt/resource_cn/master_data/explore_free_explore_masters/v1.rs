#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExploreFreeExploreMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub explore_free_explore_master_id: i64,
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    #[prost(int64, tag = "4")]
    pub energy_cost: i64,
    #[prost(int64, tag = "5")]
    pub lifetime_hours: i64,
    #[prost(
        enumeration = "super::super::super::super::super::score::resource_cn::common::v1::TriggerType",
        tag = "11"
    )]
    pub trigger_type: i32,
    #[prost(string, tag = "12")]
    pub trigger_param_1: ::prost::alloc::string::String,
    #[prost(string, tag = "13")]
    pub trigger_param_2: ::prost::alloc::string::String,
    #[prost(string, tag = "14")]
    pub trigger_param_3: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "21")]
    pub condition: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(
        enumeration = "super::super::explore_event_masters::v1::ExploreEventType",
        tag = "31"
    )]
    pub event_type: i32,
    #[prost(string, tag = "32")]
    pub event_param_1: ::prost::alloc::string::String,
    #[prost(string, tag = "33")]
    pub event_param_2: ::prost::alloc::string::String,
    #[prost(
        enumeration = "super::super::super::super::super::score::resource_cn::common::v1::LimitType",
        tag = "41"
    )]
    pub limit_type: i32,
    #[prost(int64, tag = "42")]
    pub limit_count: i64,
    #[prost(int64, tag = "43")]
    pub limit_cooldown_hours: i64,
    #[prost(string, repeated, tag = "51")]
    pub rewards: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, tag = "52")]
    pub explore_object_master_id: i64,
    #[prost(int64, tag = "53")]
    pub explore_point_master_id: i64,
    #[prost(string, repeated, tag = "54")]
    pub first_time_rewards: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, tag = "55")]
    pub drop_master_id: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExploreFreeExploreMasters {
    #[prost(message, repeated, tag = "1")]
    pub explore_free_explore_masters: ::prost::alloc::vec::Vec<ExploreFreeExploreMaster>,
}
