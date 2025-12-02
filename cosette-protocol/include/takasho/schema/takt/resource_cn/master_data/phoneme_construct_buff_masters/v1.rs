#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhonemeConstructBuffMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub phoneme_construct_buff_master_id: i64,
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    #[prost(int64, tag = "4")]
    pub weight: i64,
    #[prost(int64, repeated, tag = "5")]
    pub job_limit: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, tag = "6")]
    pub value: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhonemeConstructBuffMasters {
    #[prost(message, repeated, tag = "1")]
    pub phoneme_construct_buff_masters: ::prost::alloc::vec::Vec<
        PhonemeConstructBuffMaster,
    >,
}
