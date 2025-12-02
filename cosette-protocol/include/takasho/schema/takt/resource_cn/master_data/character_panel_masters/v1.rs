#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CharacterPanelMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub character_panel_master_id: i64,
    #[prost(int64, tag = "3")]
    pub battle_character_master_id: i64,
    #[prost(int64, tag = "4")]
    pub page_id: i64,
    #[prost(string, repeated, tag = "11")]
    pub get_attributes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, tag = "13")]
    pub get_skill_id: i64,
    #[prost(int64, tag = "14")]
    pub get_battle_character_grade: i64,
    #[prost(int64, tag = "21")]
    pub require_battle_character_lv: i64,
    #[prost(int64, repeated, tag = "22")]
    pub require_character_panel_master_ids: ::prost::alloc::vec::Vec<i64>,
    #[prost(string, repeated, tag = "23")]
    pub require_contents: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CharacterPanelMasters {
    #[prost(message, repeated, tag = "1")]
    pub character_panel_masters: ::prost::alloc::vec::Vec<CharacterPanelMaster>,
}
