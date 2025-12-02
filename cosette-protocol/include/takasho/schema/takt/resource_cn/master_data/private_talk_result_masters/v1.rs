#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivateTalkResultMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub private_talk_result_master_id: i64,
    #[prost(int64, tag = "3")]
    pub talk_point: i64,
    #[prost(int64, tag = "4")]
    pub bond_value: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivateTalkResultMasters {
    #[prost(message, repeated, tag = "1")]
    pub private_talk_result_masters: ::prost::alloc::vec::Vec<PrivateTalkResultMaster>,
}
