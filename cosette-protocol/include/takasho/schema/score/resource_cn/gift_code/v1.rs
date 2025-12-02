#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerGiftCode {
    #[prost(string, tag = "1")]
    pub player_gift_code_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub player_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub gift_code: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub batch_series_name: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub batch_text: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub batch_id: ::prost::alloc::string::String,
    #[prost(sint64, tag = "91")]
    pub created_at: i64,
    #[prost(sint64, tag = "92")]
    pub updated_at: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GiftCodeBatch {
    #[prost(string, tag = "1")]
    pub gift_code_batch_id: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub is_customized_batch: bool,
    #[prost(string, tag = "3")]
    pub batch_series_name: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub batch_text: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub parent_batch_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "11")]
    pub reuse_limit_count: i64,
    #[prost(int64, tag = "12")]
    pub user_own_codes_limit_count: i64,
    #[prost(int64, tag = "13")]
    pub codes_count: i64,
    #[prost(sint64, tag = "14")]
    pub end_time: i64,
    #[prost(string, repeated, tag = "15")]
    pub affcode_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, repeated, tag = "16")]
    pub realm_ids: ::prost::alloc::vec::Vec<i64>,
    #[prost(bool, tag = "17")]
    pub enabled: bool,
    #[prost(string, repeated, tag = "18")]
    pub rewards: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "21")]
    pub comment: ::prost::alloc::string::String,
    #[prost(sint64, tag = "91")]
    pub created_at: i64,
    #[prost(sint64, tag = "92")]
    pub updated_at: i64,
}
