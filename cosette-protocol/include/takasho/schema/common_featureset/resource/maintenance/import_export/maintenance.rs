#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Maintenances {
    #[prost(message, repeated, tag = "1")]
    pub maintenances: ::prost::alloc::vec::Vec<Maintenance>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Maintenance {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    #[prost(sint64, tag = "3")]
    pub opened_at: i64,
    #[prost(sint64, tag = "4")]
    pub closed_at: i64,
}
