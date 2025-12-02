#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActivityMissionEntryMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub activity_mission_entry_master_id: i64,
    #[prost(int64, tag = "3")]
    pub activity_mission_master_id: i64,
    #[prost(int64, tag = "4")]
    pub mission_master_id: i64,
    #[prost(int64, tag = "5")]
    pub day: i64,
    #[prost(string, repeated, tag = "6")]
    pub condition: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActivityMissionEntryMasters {
    #[prost(message, repeated, tag = "1")]
    pub activity_mission_entry_masters: ::prost::alloc::vec::Vec<
        ActivityMissionEntryMaster,
    >,
}
