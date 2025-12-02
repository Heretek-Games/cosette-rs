#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExploreAdvMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub explore_adv_master_id: i64,
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    #[prost(int64, tag = "4")]
    pub explore_event_master_id: i64,
    #[prost(bool, tag = "5")]
    pub is_section_end: bool,
    #[prost(int64, tag = "7")]
    pub clock_cost: i64,
    #[prost(string, repeated, tag = "8")]
    pub rewards: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, tag = "11")]
    pub puzzle_id: i64,
    #[prost(int64, tag = "12")]
    pub adv_battle_id: i64,
    #[prost(string, repeated, tag = "21")]
    pub bond_rewards: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExploreAdvMasters {
    #[prost(message, repeated, tag = "1")]
    pub explore_adv_masters: ::prost::alloc::vec::Vec<ExploreAdvMaster>,
}
