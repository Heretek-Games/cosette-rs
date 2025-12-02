#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoginBonuses {
    #[prost(message, repeated, tag = "1")]
    pub login_bonuses: ::prost::alloc::vec::Vec<LoginBonus>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoginBonus {
    #[prost(string, tag = "1")]
    pub login_bonus_id: ::prost::alloc::string::String,
    #[prost(sint64, tag = "2")]
    pub opened_at: i64,
    #[prost(sint64, tag = "3")]
    pub closed_at: i64,
    #[prost(enumeration = "super::super::login_bonus::v1::ResetType", tag = "4")]
    pub reset_type: i32,
    #[prost(string, tag = "5")]
    pub date_line: ::prost::alloc::string::String,
    #[prost(sint64, tag = "6")]
    pub time_lapse: i64,
    #[prost(message, repeated, tag = "7")]
    pub prizes: ::prost::alloc::vec::Vec<LoginBonusPrize>,
    #[prost(bool, tag = "8")]
    pub r#loop: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoginBonusPrize {
    #[prost(string, tag = "1")]
    pub login_bonus_prize_id: ::prost::alloc::string::String,
    #[prost(sint64, tag = "2")]
    pub prize_count: i64,
    #[prost(enumeration = "super::super::player_inventory::v1::ItemType", tag = "3")]
    pub item_type: i32,
    #[prost(string, tag = "4")]
    pub schema_key: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub value_path: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub inventory_message: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub search_label: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub system_resource_name: ::prost::alloc::string::String,
    #[prost(sint64, tag = "9")]
    pub system_resource_num: i64,
    #[prost(string, tag = "10")]
    pub transaction_payload: ::prost::alloc::string::String,
}
