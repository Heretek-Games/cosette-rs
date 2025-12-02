#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShopMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub shop_master_id: i64,
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    #[prost(int64, tag = "4")]
    pub require_player_level: i64,
    #[prost(int64, repeated, tag = "5")]
    pub activity_types: ::prost::alloc::vec::Vec<i64>,
    #[prost(bool, tag = "6")]
    pub need_config_time: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShopMasters {
    #[prost(message, repeated, tag = "1")]
    pub shop_masters: ::prost::alloc::vec::Vec<ShopMaster>,
}
