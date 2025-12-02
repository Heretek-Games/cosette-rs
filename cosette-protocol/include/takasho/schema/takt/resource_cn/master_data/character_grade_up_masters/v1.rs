#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CharacterGradeUpMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub character_grade_up_master_id: i64,
    #[prost(int64, tag = "3")]
    pub grade: i64,
    #[prost(int64, tag = "4")]
    pub level_cap: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CharacterGradeUpMasters {
    #[prost(message, repeated, tag = "1")]
    pub character_grade_up_masters: ::prost::alloc::vec::Vec<CharacterGradeUpMaster>,
}
