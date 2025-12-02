#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MonthlyCheckinRewardsMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub monthly_checkin_rewards_master_id: i64,
    #[prost(int64, tag = "3")]
    pub month: i64,
    #[prost(int64, tag = "4")]
    pub date: i64,
    #[prost(string, repeated, tag = "5")]
    pub rewards: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MonthlyCheckinRewardsMasters {
    #[prost(message, repeated, tag = "1")]
    pub monthly_checkin_rewards_masters: ::prost::alloc::vec::Vec<
        MonthlyCheckinRewardsMaster,
    >,
}
