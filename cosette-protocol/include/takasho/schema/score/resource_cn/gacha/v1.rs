#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerGacha {
    #[prost(string, tag = "1")]
    pub player_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub gacha_master_id: i64,
    #[prost(int64, tag = "3")]
    pub single_gacha_count: i64,
    #[prost(int64, tag = "4")]
    pub combo_gacha_count: i64,
    #[prost(int64, tag = "5")]
    pub no_sr_count: i64,
    #[prost(int64, tag = "6")]
    pub no_ssr_count: i64,
    #[prost(sint64, tag = "7")]
    pub last_gacha_at: i64,
    #[prost(sint64, tag = "8")]
    pub daily_limit_gacha_at: i64,
    #[prost(int64, tag = "9")]
    pub daily_single_gacha_count: i64,
    #[prost(int64, tag = "10")]
    pub daily_combo_gacha_count: i64,
    #[prost(sint64, tag = "11")]
    pub next_cycle_start_at: i64,
    #[prost(int64, tag = "13")]
    pub finish_count: i64,
    #[prost(message, optional, tag = "14")]
    pub limit: ::core::option::Option<super::super::common::v1::Limit>,
    #[prost(string, tag = "15")]
    pub limit_db: ::prost::alloc::string::String,
    #[prost(int64, tag = "16")]
    pub prefer: i64,
    #[prost(int64, tag = "17")]
    pub no_common_count: i64,
    #[prost(sint64, tag = "91")]
    pub created_at: i64,
    #[prost(sint64, tag = "92")]
    pub updated_at: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerGachaHistory {
    #[prost(string, tag = "1")]
    pub player_gacha_history_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub player_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    pub gacha_master_id: i64,
    #[prost(bool, tag = "4")]
    pub is_single: bool,
    #[prost(bool, tag = "5")]
    pub is_combo: bool,
    #[prost(bool, tag = "6")]
    pub is_daily_limit: bool,
    #[prost(string, repeated, tag = "7")]
    pub db_result: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "8")]
    pub result: ::prost::alloc::vec::Vec<player_gacha_history::Result>,
    #[prost(sint64, tag = "91")]
    pub created_at: i64,
    #[prost(sint64, tag = "92")]
    pub updated_at: i64,
}
/// Nested message and enum types in `PlayerGachaHistory`.
pub mod player_gacha_history {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Result {
        #[prost(int64, tag = "1")]
        pub id: i64,
        #[prost(int64, tag = "2")]
        pub count: i64,
        #[prost(message, repeated, tag = "3")]
        pub converted: ::prost::alloc::vec::Vec<result::ConvertedEntry>,
    }
    /// Nested message and enum types in `Result`.
    pub mod result {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ConvertedEntry {
            #[prost(int64, tag = "1")]
            pub key: i64,
            #[prost(int64, tag = "2")]
            pub value: i64,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AvailableGacha {
    #[prost(int64, tag = "1")]
    pub gacha_master_id: i64,
    #[prost(bool, tag = "2")]
    pub is_open: bool,
    #[prost(sint64, tag = "3")]
    pub open_at: i64,
    #[prost(sint64, tag = "4")]
    pub close_at: i64,
    #[prost(sint64, tag = "91")]
    pub created_at: i64,
    #[prost(sint64, tag = "92")]
    pub updated_at: i64,
}
