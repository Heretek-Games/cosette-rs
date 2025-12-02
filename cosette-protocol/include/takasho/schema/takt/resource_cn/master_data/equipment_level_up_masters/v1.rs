#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EquipmentLevelUpMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub equipment_level_up_master_id: i64,
    #[prost(int64, tag = "3")]
    pub level: i64,
    #[prost(int64, tag = "4")]
    pub exp_n: i64,
    #[prost(int64, tag = "5")]
    pub exp_r: i64,
    #[prost(int64, tag = "6")]
    pub exp_sr: i64,
    #[prost(int64, tag = "7")]
    pub exp_ssr: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EquipmentLevelUpMasters {
    #[prost(message, repeated, tag = "1")]
    pub equipment_level_up_masters: ::prost::alloc::vec::Vec<EquipmentLevelUpMaster>,
}
