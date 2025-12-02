#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Affcode {
    #[prost(string, tag = "1")]
    pub affcode_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub affcode: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub affcode_name: ::prost::alloc::string::String,
    #[prost(sint64, tag = "91")]
    pub created_at: i64,
    #[prost(sint64, tag = "92")]
    pub updated_at: i64,
}
