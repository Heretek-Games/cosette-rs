#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuffMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub buff_master_id: i64,
    #[prost(string, tag = "3")]
    pub effect_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub icon_path: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub show_text: ::prost::alloc::string::String,
    #[prost(int64, tag = "6")]
    pub target_type: i64,
    #[prost(int64, repeated, tag = "7")]
    pub effect_value: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, tag = "11")]
    pub turn: i64,
    #[prost(int64, tag = "12")]
    pub max_stock: i64,
    #[prost(int64, tag = "13")]
    pub replace_type: i64,
    #[prost(int64, tag = "14")]
    pub success_timing: i64,
    #[prost(int64, tag = "15")]
    pub success_percent: i64,
    #[prost(int64, tag = "16")]
    pub active_event: i64,
    #[prost(string, tag = "17")]
    pub active_condition: ::prost::alloc::string::String,
    #[prost(int64, repeated, tag = "18")]
    pub active_condition_param: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, tag = "19")]
    pub buff_type: i64,
    #[prost(int64, repeated, tag = "20")]
    pub child_buff_id: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, tag = "21")]
    pub disperse_type: i64,
    #[prost(int64, tag = "22")]
    pub priority: i64,
    #[prost(int64, tag = "23")]
    pub cool_down: i64,
    #[prost(int64, repeated, tag = "24")]
    pub params: ::prost::alloc::vec::Vec<i64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuffMasters {
    #[prost(message, repeated, tag = "1")]
    pub buff_masters: ::prost::alloc::vec::Vec<BuffMaster>,
}
