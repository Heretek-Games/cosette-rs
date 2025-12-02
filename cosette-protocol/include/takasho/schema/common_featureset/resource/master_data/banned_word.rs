#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BannedWords {
    #[prost(message, repeated, tag = "1")]
    pub banned_words: ::prost::alloc::vec::Vec<BannedWordSet>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BannedWordSet {
    #[prost(string, tag = "1")]
    pub language_code: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub words: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
