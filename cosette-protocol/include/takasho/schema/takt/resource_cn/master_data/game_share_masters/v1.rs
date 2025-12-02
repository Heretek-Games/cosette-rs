#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GameShareMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub game_share_master_id: i64,
    #[prost(string, repeated, tag = "3")]
    pub contents: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(
        enumeration = "super::super::super::super::super::score::resource_cn::system::v1::Region",
        tag = "4"
    )]
    pub region: i32,
    #[prost(int64, tag = "5")]
    pub mail_template_id: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GameShareMasters {
    #[prost(message, repeated, tag = "1")]
    pub game_share_masters: ::prost::alloc::vec::Vec<GameShareMaster>,
}
