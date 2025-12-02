#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PuzzleDrinkAccessoryMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub puzzle_drink_accessory_master_id: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PuzzleDrinkAccessoryMasters {
    #[prost(message, repeated, tag = "1")]
    pub puzzle_drink_accessory_masters: ::prost::alloc::vec::Vec<
        PuzzleDrinkAccessoryMaster,
    >,
}
