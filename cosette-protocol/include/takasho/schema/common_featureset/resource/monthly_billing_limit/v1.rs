#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MonthlyBillingLimit {
    #[prost(int64, tag = "1")]
    pub min_age: i64,
    #[prost(int64, tag = "2")]
    pub max_age: i64,
    #[prost(int64, tag = "3")]
    pub billing_limit: i64,
}
