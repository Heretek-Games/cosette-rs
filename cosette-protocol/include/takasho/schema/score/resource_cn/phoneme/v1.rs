#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhonemeBuffEntry {
    #[prost(int64, repeated, tag = "1")]
    pub phoneme_buff_master_ids: ::prost::alloc::vec::Vec<i64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerPhoneme {
    #[prost(string, tag = "1")]
    pub player_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub player_phoneme_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    pub phoneme_master_id: i64,
    #[prost(int64, tag = "4")]
    pub exp: i64,
    #[prost(int64, tag = "5")]
    pub grade: i64,
    #[prost(string, tag = "31")]
    pub buff_ids_values: ::prost::alloc::string::String,
    #[prost(string, tag = "39")]
    pub buff_ids_values_candidates: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "45")]
    pub res_buff_ids_values: ::prost::alloc::vec::Vec<
        player_phoneme::ResBuffIdsValuesEntry,
    >,
    #[prost(message, repeated, tag = "46")]
    pub res_buff_ids_values_candidates: ::prost::alloc::vec::Vec<
        player_phoneme::ResBuffIdsValuesCandidatesEntry,
    >,
    #[prost(int64, tag = "8")]
    pub battle_character_master_id: i64,
    #[prost(int64, tag = "9")]
    pub equipped_slot_position_index: i64,
    #[prost(int64, tag = "10")]
    pub construct_buff_master_id: i64,
    #[prost(int64, tag = "11")]
    pub pending_construct_buff_master_id: i64,
    #[prost(int64, tag = "12")]
    pub is_lock: i64,
    #[prost(sint64, tag = "91")]
    pub created_at: i64,
    #[prost(sint64, tag = "92")]
    pub updated_at: i64,
}
/// Nested message and enum types in `PlayerPhoneme`.
pub mod player_phoneme {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ResBuffIdsValuesEntry {
        #[prost(int32, tag = "1")]
        pub key: i32,
        #[prost(message, optional, tag = "2")]
        pub value: ::core::option::Option<BuffValues>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ResBuffIdsValuesCandidatesEntry {
        #[prost(int32, tag = "1")]
        pub key: i32,
        #[prost(message, optional, tag = "2")]
        pub value: ::core::option::Option<BuffValues>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BuffValues {
        #[prost(int64, repeated, tag = "1")]
        pub vs: ::prost::alloc::vec::Vec<i64>,
    }
}
