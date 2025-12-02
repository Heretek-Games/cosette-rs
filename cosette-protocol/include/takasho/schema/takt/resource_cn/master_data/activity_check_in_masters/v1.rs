#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActivityCheckInMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub activity_check_in_master_id: i64,
    #[prost(int64, tag = "5")]
    pub limit_time: i64,
    #[prost(string, repeated, tag = "6")]
    pub conditions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActivityCheckInMasters {
    #[prost(message, repeated, tag = "1")]
    pub activity_check_in_masters: ::prost::alloc::vec::Vec<ActivityCheckInMaster>,
}
