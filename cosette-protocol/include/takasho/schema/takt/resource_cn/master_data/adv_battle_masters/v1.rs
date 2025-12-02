#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdvBattleMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub adv_battle_master_id: i64,
    #[prost(int64, tag = "3")]
    pub use_stamina: i64,
    #[prost(int64, tag = "4")]
    pub battle_id: i64,
    #[prost(int64, tag = "5")]
    pub exp_player: i64,
    #[prost(int64, tag = "6")]
    pub exp_battle_character: i64,
    #[prost(int64, tag = "7")]
    pub game_money: i64,
    #[prost(int64, tag = "8")]
    pub drop_master_id: i64,
    #[prost(int64, tag = "9")]
    pub chapter: i64,
    #[prost(int64, tag = "10")]
    pub section: i64,
    #[prost(string, repeated, tag = "11")]
    pub first_clear_rewards: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdvBattleMasters {
    #[prost(message, repeated, tag = "1")]
    pub adv_battle_masters: ::prost::alloc::vec::Vec<AdvBattleMaster>,
}
