#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeatureReceivableTimeTypes {
    #[prost(message, repeated, tag = "1")]
    pub feature_receivable_time_types: ::prost::alloc::vec::Vec<
        FeatureReceivableTimeType,
    >,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeatureReceivableTimeType {
    #[prost(sint64, tag = "1")]
    pub feature_type: i64,
    #[prost(
        enumeration = "super::super::feature_receivable_time_type::v1::ReceivableTimeType",
        tag = "2"
    )]
    pub receivable_time_type: i32,
    #[prost(sint64, tag = "3")]
    pub receivable_sec: i64,
    #[prost(sint64, tag = "4")]
    pub receivable_day: i64,
}
