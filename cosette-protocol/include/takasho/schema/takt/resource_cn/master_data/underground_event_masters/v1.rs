#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UndergroundEventMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub underground_event_master_id: i64,
    #[prost(int64, tag = "3")]
    pub underground_section_master_id: i64,
    #[prost(string, tag = "11")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "12")]
    pub finish_condition: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "13")]
    pub rewards: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "14")]
    pub first_clear_rewards: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, tag = "15")]
    pub progress_weight: i64,
    #[prost(int64, tag = "22")]
    pub progress_weight2: i64,
    #[prost(bool, tag = "16")]
    pub hide: bool,
    #[prost(bool, tag = "17")]
    pub repeatable: bool,
    #[prost(bool, tag = "18")]
    pub accomplish: bool,
    #[prost(int64, tag = "19")]
    pub stamina: i64,
    #[prost(enumeration = "EventType", tag = "20")]
    pub r#type: i32,
    #[prost(int64, tag = "21")]
    pub battle_stage_master_id: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UndergroundEventMasters {
    #[prost(message, repeated, tag = "1")]
    pub underground_event_masters: ::prost::alloc::vec::Vec<UndergroundEventMaster>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EventType {
    NormalEvent = 0,
    BattleEvent = 1,
    ChestEvent = 2,
}
impl EventType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EventType::NormalEvent => "NormalEvent",
            EventType::BattleEvent => "BattleEvent",
            EventType::ChestEvent => "ChestEvent",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NormalEvent" => Some(Self::NormalEvent),
            "BattleEvent" => Some(Self::BattleEvent),
            "ChestEvent" => Some(Self::ChestEvent),
            _ => None,
        }
    }
}
