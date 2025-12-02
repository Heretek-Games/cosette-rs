#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EquipmentExchangeMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub equipment_exchange_master_id: i64,
    #[prost(string, repeated, tag = "3")]
    pub unchanged_equipments: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EquipmentExchangeMasters {
    #[prost(message, repeated, tag = "1")]
    pub equipment_exchange_masters: ::prost::alloc::vec::Vec<EquipmentExchangeMaster>,
}
