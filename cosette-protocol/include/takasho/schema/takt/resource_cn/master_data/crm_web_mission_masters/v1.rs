#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CrmWebMissionMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub crm_web_mission_master_id: i64,
    #[prost(int64, tag = "3")]
    pub mission_category: i64,
    #[prost(int64, repeated, tag = "4")]
    pub previous_mission_ids: ::prost::alloc::vec::Vec<i64>,
    #[prost(string, repeated, tag = "5")]
    pub unlock_conditions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, tag = "6")]
    pub mission_type_id: i64,
    #[prost(int64, tag = "7")]
    pub required_total_index: i64,
    #[prost(int64, tag = "8")]
    pub mission_type_value1: i64,
    #[prost(int64, tag = "9")]
    pub mission_type_value2: i64,
    #[prost(int64, tag = "10")]
    pub mission_type_value_3: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CrmWebMissionMasters {
    #[prost(message, repeated, tag = "1")]
    pub crm_web_mission_masters: ::prost::alloc::vec::Vec<CrmWebMissionMaster>,
}
