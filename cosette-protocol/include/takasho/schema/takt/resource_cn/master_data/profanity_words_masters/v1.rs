#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProfanityWordsMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub profanity_words_master_id: i64,
    #[prost(string, tag = "3")]
    pub content: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProfanityWordsMasters {
    #[prost(message, repeated, tag = "1")]
    pub profanity_words_masters: ::prost::alloc::vec::Vec<ProfanityWordsMaster>,
}
