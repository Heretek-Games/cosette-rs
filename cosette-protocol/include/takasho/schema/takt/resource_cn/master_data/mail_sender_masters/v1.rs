#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MailSenderMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub mail_sender_master_id: i64,
    #[prost(string, tag = "3")]
    pub sender_key: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub sender_key_desc: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MailSenderMasters {
    #[prost(message, repeated, tag = "1")]
    pub mail_sender_masters: ::prost::alloc::vec::Vec<MailSenderMaster>,
}
