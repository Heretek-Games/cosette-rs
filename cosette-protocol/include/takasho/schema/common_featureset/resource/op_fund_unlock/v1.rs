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
    #[prost(enumeration = "super::super::player_inventory::v1::ItemType", tag = "4")]
    pub item_type: i32,
    #[prost(string, tag = "5")]
    pub schema_key: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "6")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "7")]
    pub search_label: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub system_resource_name: ::prost::alloc::string::String,
    #[prost(sint64, tag = "9")]
    pub system_resource_num: i64,
}
