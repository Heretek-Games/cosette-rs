#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MasterSectionJudgmentMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub master_section_judgment_master_id: i64,
    #[prost(int64, tag = "3")]
    pub battle_stage_master_id: i64,
    #[prost(int64, tag = "4")]
    pub required_effect_number: i64,
    #[prost(int64, tag = "5")]
    pub cd: i64,
    #[prost(int64, tag = "6")]
    pub integral_weight: i64,
    #[prost(int64, tag = "7")]
    pub base_integral: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MasterSectionJudgmentMasters {
    #[prost(message, repeated, tag = "1")]
    pub master_section_judgment_masters: ::prost::alloc::vec::Vec<
        MasterSectionJudgmentMaster,
    >,
}
