#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CrusadeEventMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub crusade_event_master_id: i64,
    #[prost(enumeration = "CrusadeMapEventType", tag = "3")]
    pub r#type: i32,
    #[prost(int64, tag = "4")]
    pub battle_master_id: i64,
    #[prost(int64, repeated, tag = "5")]
    pub values: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, tag = "6")]
    pub drop_treasure_group_master_id: i64,
    #[prost(int64, tag = "7")]
    pub drop_master_id: i64,
    #[prost(int64, tag = "8")]
    pub next_events: i64,
    #[prost(bool, tag = "9")]
    pub free: bool,
    #[prost(int64, tag = "20")]
    pub hp_recover_value: i64,
    #[prost(int64, tag = "21")]
    pub ap_recover_value: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CrusadeEventMasters {
    #[prost(message, repeated, tag = "1")]
    pub crusade_event_masters: ::prost::alloc::vec::Vec<CrusadeEventMaster>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CrusadeMapEventType {
    InvalidCrusadeMapEventType = 0,
    CrusadeBeginningEventType = 1,
    CrusadeShieldEventType = 2,
    CrusadeBattleEventType = 3,
    CrusadeCampEventType = 4,
    CrusadeChoiceEventType = 5,
    CrusadeScenarioEventType = 6,
    CrusadeEndEventType = 7,
    CrusadeReOpenEventType = 8,
    CrusadeUndergroundType = 9,
}
impl CrusadeMapEventType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CrusadeMapEventType::InvalidCrusadeMapEventType => {
                "InvalidCrusadeMapEventType"
            }
            CrusadeMapEventType::CrusadeBeginningEventType => "CrusadeBeginningEventType",
            CrusadeMapEventType::CrusadeShieldEventType => "CrusadeShieldEventType",
            CrusadeMapEventType::CrusadeBattleEventType => "CrusadeBattleEventType",
            CrusadeMapEventType::CrusadeCampEventType => "CrusadeCampEventType",
            CrusadeMapEventType::CrusadeChoiceEventType => "CrusadeChoiceEventType",
            CrusadeMapEventType::CrusadeScenarioEventType => "CrusadeScenarioEventType",
            CrusadeMapEventType::CrusadeEndEventType => "CrusadeEndEventType",
            CrusadeMapEventType::CrusadeReOpenEventType => "CrusadeReOpenEventType",
            CrusadeMapEventType::CrusadeUndergroundType => "CrusadeUndergroundType",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "InvalidCrusadeMapEventType" => Some(Self::InvalidCrusadeMapEventType),
            "CrusadeBeginningEventType" => Some(Self::CrusadeBeginningEventType),
            "CrusadeShieldEventType" => Some(Self::CrusadeShieldEventType),
            "CrusadeBattleEventType" => Some(Self::CrusadeBattleEventType),
            "CrusadeCampEventType" => Some(Self::CrusadeCampEventType),
            "CrusadeChoiceEventType" => Some(Self::CrusadeChoiceEventType),
            "CrusadeScenarioEventType" => Some(Self::CrusadeScenarioEventType),
            "CrusadeEndEventType" => Some(Self::CrusadeEndEventType),
            "CrusadeReOpenEventType" => Some(Self::CrusadeReOpenEventType),
            "CrusadeUndergroundType" => Some(Self::CrusadeUndergroundType),
            _ => None,
        }
    }
}
