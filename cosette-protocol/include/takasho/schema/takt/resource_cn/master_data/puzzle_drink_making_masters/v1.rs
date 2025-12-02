#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PuzzleDrinkMakingMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub puzzle_drink_making_master_id: i64,
    #[prost(int64, tag = "3")]
    pub puzzle_explore_master_id: i64,
    #[prost(string, tag = "4")]
    pub target_drink: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "5")]
    pub costs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, repeated, tag = "6")]
    pub favored_accessories: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, repeated, tag = "7")]
    pub normal_accessories: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, repeated, tag = "8")]
    pub unsuitable_accessories: ::prost::alloc::vec::Vec<i64>,
    #[prost(string, repeated, tag = "11")]
    pub s_level_rewards: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "12")]
    pub a_level_rewards: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "13")]
    pub b_level_rewards: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PuzzleDrinkMakingMasters {
    #[prost(message, repeated, tag = "1")]
    pub puzzle_drink_making_masters: ::prost::alloc::vec::Vec<PuzzleDrinkMakingMaster>,
}
