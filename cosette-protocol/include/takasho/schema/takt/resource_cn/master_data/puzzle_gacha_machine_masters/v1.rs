#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PuzzleGachaMachineMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub puzzle_gacha_machine_master_id: i64,
    #[prost(int64, tag = "3")]
    pub gacha_item_id: i64,
    #[prost(int64, tag = "4")]
    pub trigger_weight: i64,
    #[prost(string, tag = "5")]
    pub scenario_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "6")]
    pub rewards: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PuzzleGachaMachineMasters {
    #[prost(message, repeated, tag = "1")]
    pub puzzle_gacha_machine_masters: ::prost::alloc::vec::Vec<PuzzleGachaMachineMaster>,
}
