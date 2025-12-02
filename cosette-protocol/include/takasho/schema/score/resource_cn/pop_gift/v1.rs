#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerPopGift {
    #[prost(string, tag = "1")]
    pub player_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub shop_pop_gift_master_id: i64,
    #[prost(int64, tag = "4")]
    pub buy_count: i64,
    #[prost(int64, tag = "5")]
    pub expired_count: i64,
    #[prost(sint64, tag = "11")]
    pub start_pop_at: i64,
    #[prost(sint64, tag = "12")]
    pub end_pop_at: i64,
    #[prost(sint64, tag = "13")]
    pub purchased_at: i64,
    #[prost(sint64, tag = "91")]
    pub created_at: i64,
    #[prost(sint64, tag = "92")]
    pub updated_at: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerPopGiftHistory {
    #[prost(string, tag = "1")]
    pub player_id: ::prost::alloc::string::String,
    #[prost(
        enumeration = "super::super::super::super::takt::resource_cn::master_data::shop_pop_gift_masters::v1::GiftPopType",
        tag = "2"
    )]
    pub pop_type: i32,
    #[prost(int64, tag = "3")]
    pub stamina_cost_count: i64,
    #[prost(int64, tag = "4")]
    pub battle_lose_count: i64,
    #[prost(sint64, tag = "11")]
    pub last_updated_at: i64,
    #[prost(sint64, tag = "91")]
    pub created_at: i64,
    #[prost(sint64, tag = "92")]
    pub updated_at: i64,
}
