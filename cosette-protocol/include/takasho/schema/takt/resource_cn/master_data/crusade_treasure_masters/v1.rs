#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CrusadeTreasureMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub crusade_treasure_master_id: i64,
    #[prost(enumeration = "CrusadeTreasureKind", tag = "3")]
    pub kind: i32,
    #[prost(int64, tag = "4")]
    pub times: i64,
    #[prost(int64, tag = "5")]
    pub active_event_type: i64,
    #[prost(int64, tag = "6")]
    pub work_on_type: i64,
    #[prost(enumeration = "CrusadeTreasureType", tag = "7")]
    pub r#type: i32,
    #[prost(int64, tag = "8")]
    pub relation_plain: i64,
    #[prost(int64, tag = "9")]
    pub relation_percent: i64,
    #[prost(int64, tag = "10")]
    pub final_times: i64,
    #[prost(int64, tag = "11")]
    pub initial_times: i64,
    #[prost(enumeration = "CrusadeTreasureActiveType", tag = "12")]
    pub active_type: i32,
    #[prost(string, repeated, tag = "13")]
    pub unlock: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, tag = "14")]
    pub step_length: i64,
    #[prost(int64, tag = "15")]
    pub max_active_times: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CrusadeTreasureMasters {
    #[prost(message, repeated, tag = "1")]
    pub crusade_treasure_masters: ::prost::alloc::vec::Vec<CrusadeTreasureMaster>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CrusadeTreasureType {
    InvalidCrusadeTreasureType = 0,
    ForBattle = 1,
    ForCoin = 2,
    ForTreasure = 3,
    ForHpRecover = 6,
    ForApRecover = 7,
    ForBattleDrop = 8,
}
impl CrusadeTreasureType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CrusadeTreasureType::InvalidCrusadeTreasureType => {
                "InvalidCrusadeTreasureType"
            }
            CrusadeTreasureType::ForBattle => "CrusadeTreasureTypeForBattle",
            CrusadeTreasureType::ForCoin => "CrusadeTreasureTypeForCoin",
            CrusadeTreasureType::ForTreasure => "CrusadeTreasureTypeForTreasure",
            CrusadeTreasureType::ForHpRecover => "CrusadeTreasureTypeForHpRecover",
            CrusadeTreasureType::ForApRecover => "CrusadeTreasureTypeForApRecover",
            CrusadeTreasureType::ForBattleDrop => "CrusadeTreasureTypeForBattleDrop",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "InvalidCrusadeTreasureType" => Some(Self::InvalidCrusadeTreasureType),
            "CrusadeTreasureTypeForBattle" => Some(Self::ForBattle),
            "CrusadeTreasureTypeForCoin" => Some(Self::ForCoin),
            "CrusadeTreasureTypeForTreasure" => Some(Self::ForTreasure),
            "CrusadeTreasureTypeForHpRecover" => Some(Self::ForHpRecover),
            "CrusadeTreasureTypeForApRecover" => Some(Self::ForApRecover),
            "CrusadeTreasureTypeForBattleDrop" => Some(Self::ForBattleDrop),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CrusadeTreasureKind {
    InvalidCrusadeTreasureKind = 0,
    Common = 1,
    Env = 2,
    Single = 3,
    Ephemeral = 4,
    Invisible = 5,
}
impl CrusadeTreasureKind {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CrusadeTreasureKind::InvalidCrusadeTreasureKind => {
                "InvalidCrusadeTreasureKind"
            }
            CrusadeTreasureKind::Common => "CrusadeTreasureKindCommon",
            CrusadeTreasureKind::Env => "CrusadeTreasureKindEnv",
            CrusadeTreasureKind::Single => "CrusadeTreasureKindSingle",
            CrusadeTreasureKind::Ephemeral => "CrusadeTreasureKindEphemeral",
            CrusadeTreasureKind::Invisible => "CrusadeTreasureKindInvisible",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "InvalidCrusadeTreasureKind" => Some(Self::InvalidCrusadeTreasureKind),
            "CrusadeTreasureKindCommon" => Some(Self::Common),
            "CrusadeTreasureKindEnv" => Some(Self::Env),
            "CrusadeTreasureKindSingle" => Some(Self::Single),
            "CrusadeTreasureKindEphemeral" => Some(Self::Ephemeral),
            "CrusadeTreasureKindInvisible" => Some(Self::Invisible),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CrusadeTreasureActiveType {
    InvalidCrusadeTreasureActiveType = 0,
    CrusadeTreasureAllActive = 1,
    CrusadeTreasureReceiveActive = 2,
}
impl CrusadeTreasureActiveType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CrusadeTreasureActiveType::InvalidCrusadeTreasureActiveType => {
                "InvalidCrusadeTreasureActiveType"
            }
            CrusadeTreasureActiveType::CrusadeTreasureAllActive => {
                "CrusadeTreasureAllActive"
            }
            CrusadeTreasureActiveType::CrusadeTreasureReceiveActive => {
                "CrusadeTreasureReceiveActive"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "InvalidCrusadeTreasureActiveType" => {
                Some(Self::InvalidCrusadeTreasureActiveType)
            }
            "CrusadeTreasureAllActive" => Some(Self::CrusadeTreasureAllActive),
            "CrusadeTreasureReceiveActive" => Some(Self::CrusadeTreasureReceiveActive),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CrusadeTreasureUnlockType {
    InvalidCrusadeTreasureUnlockType = 0,
    CrusadeTreasureUnlockByTotalCount = 1,
    CrusadeTreasureUnlockByCoinCount = 2,
    CrusadeTreasureUnlockByTreasureCount = 3,
    CrusadeTreasureUnlockBySpecialLevelCount = 4,
    CrusadeTreasureUnlockByDeadCount = 5,
}
impl CrusadeTreasureUnlockType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CrusadeTreasureUnlockType::InvalidCrusadeTreasureUnlockType => {
                "InvalidCrusadeTreasureUnlockType"
            }
            CrusadeTreasureUnlockType::CrusadeTreasureUnlockByTotalCount => {
                "CrusadeTreasureUnlockByTotalCount"
            }
            CrusadeTreasureUnlockType::CrusadeTreasureUnlockByCoinCount => {
                "CrusadeTreasureUnlockByCoinCount"
            }
            CrusadeTreasureUnlockType::CrusadeTreasureUnlockByTreasureCount => {
                "CrusadeTreasureUnlockByTreasureCount"
            }
            CrusadeTreasureUnlockType::CrusadeTreasureUnlockBySpecialLevelCount => {
                "CrusadeTreasureUnlockBySpecialLevelCount"
            }
            CrusadeTreasureUnlockType::CrusadeTreasureUnlockByDeadCount => {
                "CrusadeTreasureUnlockByDeadCount"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "InvalidCrusadeTreasureUnlockType" => {
                Some(Self::InvalidCrusadeTreasureUnlockType)
            }
            "CrusadeTreasureUnlockByTotalCount" => {
                Some(Self::CrusadeTreasureUnlockByTotalCount)
            }
            "CrusadeTreasureUnlockByCoinCount" => {
                Some(Self::CrusadeTreasureUnlockByCoinCount)
            }
            "CrusadeTreasureUnlockByTreasureCount" => {
                Some(Self::CrusadeTreasureUnlockByTreasureCount)
            }
            "CrusadeTreasureUnlockBySpecialLevelCount" => {
                Some(Self::CrusadeTreasureUnlockBySpecialLevelCount)
            }
            "CrusadeTreasureUnlockByDeadCount" => {
                Some(Self::CrusadeTreasureUnlockByDeadCount)
            }
            _ => None,
        }
    }
}
