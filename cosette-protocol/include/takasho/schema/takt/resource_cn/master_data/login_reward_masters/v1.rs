#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoginRewardMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub login_reward_master_id: i64,
    #[prost(int64, tag = "3")]
    pub login_activity_master_id: i64,
    #[prost(int64, tag = "4")]
    pub day: i64,
    #[prost(string, repeated, tag = "5")]
    pub rewards: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoginRewardMasters {
    #[prost(message, repeated, tag = "1")]
    pub login_reward_masters: ::prost::alloc::vec::Vec<LoginRewardMaster>,
}
