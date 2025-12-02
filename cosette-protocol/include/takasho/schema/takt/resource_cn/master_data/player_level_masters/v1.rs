#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerLevelMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub player_level_master_id: i64,
    #[prost(int64, tag = "3")]
    pub level: i64,
    #[prost(int64, tag = "4")]
    pub exp: i64,
    #[prost(int64, tag = "5")]
    pub stamina_limit: i64,
    #[prost(int64, tag = "6")]
    pub add_stamina: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerLevelMasters {
    #[prost(message, repeated, tag = "1")]
    pub player_level_masters: ::prost::alloc::vec::Vec<PlayerLevelMaster>,
}
