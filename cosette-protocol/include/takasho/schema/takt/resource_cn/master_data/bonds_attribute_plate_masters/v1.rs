#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BondsAttributePlateMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub bonds_attribute_plate_master_id: i64,
    #[prost(int64, tag = "3")]
    pub plate_id: i64,
    #[prost(int64, tag = "4")]
    pub plate_lv: i64,
    #[prost(string, repeated, tag = "5")]
    pub resources_required: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "6")]
    pub attribute_growth: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BondsAttributePlateMasters {
    #[prost(message, repeated, tag = "1")]
    pub bonds_attribute_plate_masters: ::prost::alloc::vec::Vec<
        BondsAttributePlateMaster,
    >,
}
