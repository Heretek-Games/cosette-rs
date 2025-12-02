#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BondsLvUpMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub bonds_lv_up_master_id: i64,
    #[prost(int64, tag = "3")]
    pub lv: i64,
    #[prost(int64, tag = "4")]
    pub exp: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BondsLvUpMasters {
    #[prost(message, repeated, tag = "1")]
    pub bonds_lv_up_masters: ::prost::alloc::vec::Vec<BondsLvUpMaster>,
}
