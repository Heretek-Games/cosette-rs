#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerEquipment {
    #[prost(string, tag = "1")]
    pub player_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub player_equipment_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    pub equipment_master_id: i64,
    #[prost(int64, tag = "4")]
    pub battle_character_master_id: i64,
    #[prost(int64, tag = "5")]
    pub exp: i64,
    #[prost(bool, tag = "6")]
    pub is_lock: bool,
    #[prost(int64, tag = "7")]
    pub grade: i64,
    #[prost(int64, tag = "8")]
    pub rank: i64,
    #[prost(bool, tag = "9")]
    pub get_for_first_time: bool,
    #[prost(sint64, tag = "91")]
    pub created_at: i64,
    #[prost(sint64, tag = "92")]
    pub updated_at: i64,
}
