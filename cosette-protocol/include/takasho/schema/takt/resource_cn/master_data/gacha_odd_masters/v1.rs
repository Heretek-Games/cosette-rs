#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GachaOddMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub gacha_odd_master_id: i64,
    #[prost(int64, tag = "3")]
    pub gacha_master_id: i64,
    #[prost(int64, tag = "4")]
    pub first_single_weight: i64,
    #[prost(int64, tag = "5")]
    pub single_weight: i64,
    #[prost(int64, tag = "6")]
    pub first_combo_weight: i64,
    #[prost(int64, tag = "7")]
    pub combo_weight: i64,
    #[prost(int64, tag = "8")]
    pub sr_guaranteed_weight: i64,
    #[prost(bool, tag = "9")]
    pub can_up_weight: bool,
    #[prost(bool, tag = "10")]
    pub choose: bool,
    #[prost(bool, tag = "11")]
    pub is_ssr_pool: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GachaOddMasters {
    #[prost(message, repeated, tag = "1")]
    pub gacha_odd_masters: ::prost::alloc::vec::Vec<GachaOddMaster>,
}
