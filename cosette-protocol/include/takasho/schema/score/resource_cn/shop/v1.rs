#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerPurchaseRecord {
    #[prost(string, tag = "1")]
    pub player_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    pub product_id: i64,
    #[prost(int64, tag = "4")]
    pub total_purchase_count: i64,
    #[prost(int64, tag = "5")]
    pub current_purchase_count: i64,
    #[prost(sint64, tag = "6")]
    pub refreshed_at: i64,
    #[prost(sint64, tag = "91")]
    pub created_at: i64,
    #[prost(sint64, tag = "92")]
    pub updated_at: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShopActiveTime {
    #[prost(int64, tag = "1")]
    pub shop_master_id: i64,
    #[prost(sint64, tag = "2")]
    pub start_at: i64,
    #[prost(sint64, tag = "3")]
    pub end_at: i64,
    #[prost(sint64, tag = "4")]
    pub pre_start_at: i64,
    #[prost(sint64, tag = "91")]
    pub created_at: i64,
    #[prost(sint64, tag = "92")]
    pub updated_at: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductActiveTime {
    #[prost(int64, tag = "1")]
    pub product_master_id: i64,
    #[prost(sint64, tag = "2")]
    pub start_at: i64,
    #[prost(sint64, tag = "3")]
    pub end_at: i64,
    #[prost(sint64, tag = "91")]
    pub created_at: i64,
    #[prost(sint64, tag = "92")]
    pub updated_at: i64,
}
