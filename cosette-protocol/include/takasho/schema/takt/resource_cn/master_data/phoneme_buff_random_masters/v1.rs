#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhonemeBuffRandomMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub phoneme_buff_random_master_id: i64,
    #[prost(int64, tag = "3")]
    pub buff_id: i64,
    #[prost(int64, tag = "4")]
    pub rarity: i64,
    #[prost(string, repeated, tag = "5")]
    pub attribute_range: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "6")]
    pub reconsitution: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhonemeBuffRandomMasters {
    #[prost(message, repeated, tag = "1")]
    pub phoneme_buff_random_masters: ::prost::alloc::vec::Vec<PhonemeBuffRandomMaster>,
}
