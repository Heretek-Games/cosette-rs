#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EquipmentRankUpMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub equipment_rank_up_master_id: i64,
    #[prost(int64, tag = "3")]
    pub equipment_master_id: i64,
    #[prost(int64, tag = "4")]
    pub rank: i64,
    #[prost(string, repeated, tag = "6")]
    pub require_contents: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "61")]
    pub require_equipments: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, tag = "7")]
    pub hp_rank_up_value: i64,
    #[prost(int64, tag = "8")]
    pub hp_rank_up_add_value: i64,
    #[prost(int64, tag = "9")]
    pub atk_rank_up_value: i64,
    #[prost(int64, tag = "10")]
    pub atk_rank_up_add_value: i64,
    #[prost(int64, tag = "11")]
    pub def_rank_up_value: i64,
    #[prost(int64, tag = "12")]
    pub def_rank_up_add_value: i64,
    #[prost(int64, tag = "13")]
    pub int_rank_up_value: i64,
    #[prost(int64, tag = "14")]
    pub int_rank_up_add_value: i64,
    #[prost(int64, tag = "15")]
    pub res_rank_up_value: i64,
    #[prost(int64, tag = "16")]
    pub res_rank_up_add_value: i64,
    #[prost(int64, tag = "17")]
    pub dex_rank_up_value: i64,
    #[prost(int64, tag = "18")]
    pub dex_rank_up_add_value: i64,
    #[prost(int64, tag = "19")]
    pub crt_rank_up_value: i64,
    #[prost(int64, tag = "20")]
    pub crt_rank_up_add_value: i64,
    #[prost(int64, tag = "21")]
    pub crd_rank_up_value: i64,
    #[prost(int64, tag = "22")]
    pub crd_rank_up_add_value: i64,
    #[prost(int64, tag = "23")]
    pub acrt_rank_up_value: i64,
    #[prost(int64, tag = "24")]
    pub acrt_rank_up_add_value: i64,
    #[prost(int64, tag = "25")]
    pub crdr_rank_up_value: i64,
    #[prost(int64, tag = "26")]
    pub crdr_rank_up_add_value: i64,
    #[prost(int64, tag = "27")]
    pub skill_id: i64,
    #[prost(string, repeated, tag = "28")]
    pub decompose_content: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EquipmentRankUpMasters {
    #[prost(message, repeated, tag = "1")]
    pub equipment_rank_up_masters: ::prost::alloc::vec::Vec<EquipmentRankUpMaster>,
}
