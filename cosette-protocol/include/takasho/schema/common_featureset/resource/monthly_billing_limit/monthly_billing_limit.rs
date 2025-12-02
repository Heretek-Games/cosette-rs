#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MonthlyBillingLimits {
    #[prost(message, repeated, tag = "1")]
    pub monthly_billing_limits: ::prost::alloc::vec::Vec<MonthlyBillingLimitSet>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MonthlyBillingLimitSet {
    #[prost(int64, tag = "1")]
    pub min_age: i64,
    #[prost(int64, tag = "2")]
    pub max_age: i64,
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    #[prost(int64, tag = "4")]
    pub billing_limit: i64,
}
