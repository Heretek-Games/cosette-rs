#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhonemeGradeUpMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub phoneme_grade_up_master_id: i64,
    #[prost(int64, tag = "3")]
    pub phoneme_master_id: i64,
    #[prost(int64, tag = "4")]
    pub grade: i64,
    #[prost(int64, tag = "5")]
    pub level_cap: i64,
    #[prost(string, repeated, tag = "6")]
    pub require_contents: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, tag = "7")]
    pub hp_grade_up_value: i64,
    #[prost(int64, tag = "8")]
    pub atk_grade_up_value: i64,
    #[prost(int64, tag = "9")]
    pub def_grade_up_value: i64,
    #[prost(int64, tag = "10")]
    pub int_grade_up_value: i64,
    #[prost(int64, tag = "11")]
    pub res_grade_up_value: i64,
    #[prost(int64, tag = "12")]
    pub dex_grade_up_value: i64,
    #[prost(int64, tag = "13")]
    pub crt_grade_up_value: i64,
    #[prost(int64, tag = "14")]
    pub crd_grade_up_value: i64,
    #[prost(int64, tag = "15")]
    pub acrt_grade_up_value: i64,
    #[prost(int64, tag = "16")]
    pub crdr_grade_up_value: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhonemeGradeUpMasters {
    #[prost(message, repeated, tag = "1")]
    pub phoneme_grade_up_masters: ::prost::alloc::vec::Vec<PhonemeGradeUpMaster>,
}
