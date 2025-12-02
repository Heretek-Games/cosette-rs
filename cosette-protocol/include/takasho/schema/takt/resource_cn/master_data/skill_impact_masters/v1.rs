#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SkillImpactMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub skill_impact_master_id: i64,
    #[prost(string, tag = "3")]
    pub effect_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "4")]
    pub effect_value_1: i64,
    #[prost(int64, tag = "5")]
    pub effect_value_2: i64,
    #[prost(int64, tag = "6")]
    pub effect_value_3: i64,
    #[prost(int64, tag = "7")]
    pub effect_value_4: i64,
    #[prost(int64, tag = "8")]
    pub effect_value_5: i64,
    #[prost(int64, tag = "9")]
    pub effect_value_6: i64,
    #[prost(int64, tag = "10")]
    pub effect_value_7: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SkillImpactMasters {
    #[prost(message, repeated, tag = "1")]
    pub skill_impact_masters: ::prost::alloc::vec::Vec<SkillImpactMaster>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SkillImpactType {
    Invalid = 0,
    AoeDamageCorrection = 1,
    AttrDiffCorrection = 2,
    DistanceDamageCorrection = 3,
    BuffCorrection = 4,
    AttrCorrection = 5,
}
impl SkillImpactType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SkillImpactType::Invalid => "SkillImpactTypeInvalid",
            SkillImpactType::AoeDamageCorrection => "AoeDamageCorrection",
            SkillImpactType::AttrDiffCorrection => "AttrDiffCorrection",
            SkillImpactType::DistanceDamageCorrection => "DistanceDamageCorrection",
            SkillImpactType::BuffCorrection => "BuffCorrection",
            SkillImpactType::AttrCorrection => "AttrCorrection",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SkillImpactTypeInvalid" => Some(Self::Invalid),
            "AoeDamageCorrection" => Some(Self::AoeDamageCorrection),
            "AttrDiffCorrection" => Some(Self::AttrDiffCorrection),
            "DistanceDamageCorrection" => Some(Self::DistanceDamageCorrection),
            "BuffCorrection" => Some(Self::BuffCorrection),
            "AttrCorrection" => Some(Self::AttrCorrection),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SkillImpactValueUpdateType {
    Up = 0,
    Down = 1,
}
impl SkillImpactValueUpdateType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SkillImpactValueUpdateType::Up => "Up",
            SkillImpactValueUpdateType::Down => "Down",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Up" => Some(Self::Up),
            "Down" => Some(Self::Down),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SkillImpactValueCompareType {
    Gt = 0,
    Lt = 1,
}
impl SkillImpactValueCompareType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SkillImpactValueCompareType::Gt => "Gt",
            SkillImpactValueCompareType::Lt => "Lt",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Gt" => Some(Self::Gt),
            "Lt" => Some(Self::Lt),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SkillImpactValueCorrectType {
    CritCorrect = 0,
    DamageCorrect = 1,
    CureCorrect = 2,
}
impl SkillImpactValueCorrectType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SkillImpactValueCorrectType::CritCorrect => "CritCorrect",
            SkillImpactValueCorrectType::DamageCorrect => "DamageCorrect",
            SkillImpactValueCorrectType::CureCorrect => "CureCorrect",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CritCorrect" => Some(Self::CritCorrect),
            "DamageCorrect" => Some(Self::DamageCorrect),
            "CureCorrect" => Some(Self::CureCorrect),
            _ => None,
        }
    }
}
