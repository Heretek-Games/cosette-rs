#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BattleActivity {
    #[prost(int64, tag = "2")]
    pub battle_activity_master_id: i64,
    #[prost(string, tag = "5")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub start_time: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub end_time: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub show_start_time: ::prost::alloc::string::String,
    #[prost(string, tag = "12")]
    pub show_end_time: ::prost::alloc::string::String,
}
