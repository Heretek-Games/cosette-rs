#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GmtoolLogs {
    #[prost(string, tag = "1")]
    pub gmtool_log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub req_url: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub req_body: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub operator_email: ::prost::alloc::string::String,
    #[prost(sint64, tag = "5")]
    pub req_time: i64,
    #[prost(sint64, tag = "91")]
    pub created_at: i64,
    #[prost(sint64, tag = "92")]
    pub updated_at: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperatorImpl {
    #[prost(string, tag = "1")]
    pub email: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub roles: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
