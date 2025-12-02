#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RechargeRewardMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub recharge_reward_master_id: i64,
    #[prost(string, repeated, tag = "3")]
    pub rewards: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, tag = "4")]
    pub mail_template_id: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RechargeRewardMasters {
    #[prost(message, repeated, tag = "1")]
    pub recharge_reward_masters: ::prost::alloc::vec::Vec<RechargeRewardMaster>,
}
