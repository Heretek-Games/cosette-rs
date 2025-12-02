#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdvSpineMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub adv_spine_master_id: i64,
    #[prost(string, tag = "11")]
    pub object_name: ::prost::alloc::string::String,
    #[prost(string, tag = "12")]
    pub instruction: ::prost::alloc::string::String,
    #[prost(string, tag = "13")]
    pub body: ::prost::alloc::string::String,
    #[prost(string, tag = "14")]
    pub expression: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdvSpineMasters {
    #[prost(message, repeated, tag = "1")]
    pub adv_spine_masters: ::prost::alloc::vec::Vec<AdvSpineMaster>,
}
