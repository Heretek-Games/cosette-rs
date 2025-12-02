#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActivityMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub activity_master_id: i64,
    #[prost(int64, tag = "3")]
    pub battle_character_master_id: i64,
    #[prost(int64, tag = "4")]
    pub activity_check_in_master_id: i64,
    #[prost(int64, tag = "5")]
    pub activity_mission_master_id: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActivityMasters {
    #[prost(message, repeated, tag = "1")]
    pub activity_masters: ::prost::alloc::vec::Vec<ActivityMaster>,
}
