#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OndemandMasters {
    #[prost(message, repeated, tag = "1")]
    pub ondemand_master_entries: ::prost::alloc::vec::Vec<OndemandMasterEntry>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OndemandMasterEntry {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub value_path: ::prost::alloc::string::String,
    #[prost(sint64, tag = "3")]
    pub opened_at: i64,
    #[prost(sint64, tag = "4")]
    pub closed_at: i64,
}
