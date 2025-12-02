#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActivityCheckInEntryMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub activity_check_in_entry_master_id: i64,
    #[prost(int64, tag = "3")]
    pub activity_check_in_master_id: i64,
    #[prost(string, repeated, tag = "4")]
    pub rewards: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActivityCheckInEntryMasters {
    #[prost(message, repeated, tag = "1")]
    pub activity_check_in_entry_masters: ::prost::alloc::vec::Vec<
        ActivityCheckInEntryMaster,
    >,
}
