#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpFundUnlocks {
    #[prost(message, repeated, tag = "1")]
    pub op_fund_unlocks: ::prost::alloc::vec::Vec<OpFundUnlock>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpFundUnlock {
    #[prost(string, tag = "1")]
    pub op_event_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub op_fund_unlock_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub objective_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "4")]
    pub prizes: ::prost::alloc::vec::Vec<OpFundUnlockPrize>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpFundUnlockPrize {
    #[prost(string, tag = "1")]
    pub op_fund_unlock_prize_id: ::prost::alloc::string::String,
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
