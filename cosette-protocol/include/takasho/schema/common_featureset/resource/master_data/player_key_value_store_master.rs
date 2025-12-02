#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerKeyValueStoreMasters {
    #[prost(message, repeated, tag = "1")]
    pub player_key_value_store_masters: ::prost::alloc::vec::Vec<
        PlayerKeyValueStoreMaster,
    >,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerKeyValueStoreMaster {
    #[prost(string, tag = "1")]
    pub player_key_value_store_key: ::prost::alloc::string::String,
    #[prost(sint64, tag = "2")]
    pub expired_at: i64,
}
