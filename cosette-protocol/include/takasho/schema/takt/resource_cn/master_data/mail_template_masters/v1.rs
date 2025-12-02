#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MailTemplateMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub mail_template_master_id: i64,
    #[prost(int64, tag = "3")]
    pub mail_sender_id: i64,
    #[prost(string, tag = "4")]
    pub mail_template_title: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub mail_template_content: ::prost::alloc::string::String,
    #[prost(int64, tag = "6")]
    pub life_time_before_read: i64,
    #[prost(int64, tag = "7")]
    pub life_time_after_read: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MailTemplateMasters {
    #[prost(message, repeated, tag = "1")]
    pub mail_template_masters: ::prost::alloc::vec::Vec<MailTemplateMaster>,
}
