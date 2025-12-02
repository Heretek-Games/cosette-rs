#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Achievements {
    #[prost(message, repeated, tag = "1")]
    pub achievements: ::prost::alloc::vec::Vec<Achievement>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Achievement {
    #[prost(string, tag = "1")]
    pub achievement_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub category: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub value_path: ::prost::alloc::string::String,
    #[prost(sint64, tag = "4")]
    pub opened_at: i64,
    #[prost(sint64, tag = "5")]
    pub closed_at: i64,
    #[prost(enumeration = "super::super::achievement::v1::ResetType", tag = "6")]
    pub reset_type: i32,
    #[prost(string, tag = "7")]
    pub date_line: ::prost::alloc::string::String,
    #[prost(sint64, tag = "8")]
    pub day_of_week: i64,
    #[prost(message, repeated, tag = "9")]
    pub prizes: ::prost::alloc::vec::Vec<AchievementPrize>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AchievementPrize {
    #[prost(string, tag = "1")]
    pub achievement_prize_id: ::prost::alloc::string::String,
    #[prost(enumeration = "super::super::player_inventory::v1::ItemType", tag = "2")]
    pub item_type: i32,
    #[prost(string, tag = "3")]
    pub schema_key: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub value_path: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub inventory_message: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub search_label: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub system_resource_name: ::prost::alloc::string::String,
    #[prost(sint64, tag = "8")]
    pub system_resource_num: i64,
    #[prost(string, tag = "9")]
    pub transaction_payload: ::prost::alloc::string::String,
}
