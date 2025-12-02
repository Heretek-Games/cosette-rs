#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GachaContentMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub gacha_content_master_id: i64,
    #[prost(int64, tag = "3")]
    pub gacha_odd_master_id: i64,
    #[prost(int64, tag = "4")]
    pub resource_id: i64,
    #[prost(int64, tag = "5")]
    pub weight: i64,
    #[prost(string, tag = "6")]
    pub show_icon: ::prost::alloc::string::String,
    #[prost(bool, tag = "7")]
    pub reset_guarantee: bool,
    #[prost(bool, tag = "8")]
    pub reset_common_guarantee: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GachaContentMasters {
    #[prost(message, repeated, tag = "1")]
    pub gacha_content_masters: ::prost::alloc::vec::Vec<GachaContentMaster>,
}
