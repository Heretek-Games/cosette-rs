#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BattleCharacterGrowthMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub battle_character_growth_master_id: i64,
    #[prost(int64, tag = "3")]
    pub character_id: i64,
    #[prost(int64, tag = "11")]
    pub min_level: i64,
    #[prost(int64, tag = "12")]
    pub max_level: i64,
    #[prost(int64, tag = "13")]
    pub incr_hp: i64,
    #[prost(int64, tag = "14")]
    pub incr_atk: i64,
    #[prost(int64, tag = "15")]
    pub incr_def: i64,
    #[prost(int64, tag = "16")]
    pub incr_int: i64,
    #[prost(int64, tag = "17")]
    pub incr_res: i64,
    #[prost(int64, tag = "18")]
    pub incr_dex: i64,
    #[prost(int64, tag = "19")]
    pub incr_crt: i64,
    #[prost(int64, tag = "20")]
    pub incr_crd: i64,
    #[prost(int64, tag = "21")]
    pub incr_acrt: i64,
    #[prost(int64, tag = "22")]
    pub incr_crdr: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BattleCharacterGrowthMasters {
    #[prost(message, repeated, tag = "1")]
    pub battle_character_growth_masters: ::prost::alloc::vec::Vec<
        BattleCharacterGrowthMaster,
    >,
}
