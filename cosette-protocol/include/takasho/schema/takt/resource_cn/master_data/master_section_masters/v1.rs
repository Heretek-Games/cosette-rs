#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MasterSectionMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub master_section_master_id: i64,
    #[prost(int64, tag = "3")]
    pub next_id: i64,
    #[prost(int64, repeated, tag = "4")]
    pub judgement_master_ids: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, repeated, tag = "5")]
    pub effect_master_ids: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, repeated, tag = "6")]
    pub side_effect_master_ids: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, repeated, tag = "7")]
    pub judgment_mission_id: ::prost::alloc::vec::Vec<i64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MasterSectionMasters {
    #[prost(message, repeated, tag = "1")]
    pub master_section_masters: ::prost::alloc::vec::Vec<MasterSectionMaster>,
}
