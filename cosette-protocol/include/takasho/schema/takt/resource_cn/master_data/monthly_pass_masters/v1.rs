#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MonthlyPassMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub monthly_pass_master_id: i64,
    #[prost(string, repeated, tag = "3")]
    pub bonus: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "4")]
    pub contents: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(
        enumeration = "super::super::super::super::super::score::resource_cn::system::v1::Region",
        tag = "5"
    )]
    pub region: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MonthlyPassMasters {
    #[prost(message, repeated, tag = "1")]
    pub monthly_pass_masters: ::prost::alloc::vec::Vec<MonthlyPassMaster>,
}
