#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MasterSectionEffectMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub master_section_effect_master_id: i64,
    #[prost(enumeration = "MasterSectionEffectType", tag = "3")]
    pub r#type: i32,
    #[prost(int64, repeated, tag = "4")]
    pub skill_master_ids: ::prost::alloc::vec::Vec<i64>,
    #[prost(string, repeated, tag = "5")]
    pub unlock_conditions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, tag = "6")]
    pub integral: i64,
    #[prost(int64, repeated, tag = "7")]
    pub patch_ids: ::prost::alloc::vec::Vec<i64>,
    #[prost(enumeration = "AttachType", tag = "8")]
    pub attach_type: i32,
    #[prost(int64, tag = "9")]
    pub attach_id: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MasterSectionEffectMasters {
    #[prost(message, repeated, tag = "1")]
    pub master_section_effect_masters: ::prost::alloc::vec::Vec<
        MasterSectionEffectMaster,
    >,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MasterSectionEffectType {
    InvalidMasterSectionEffectType = 0,
    Positive = 1,
    Negative = 2,
}
impl MasterSectionEffectType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MasterSectionEffectType::InvalidMasterSectionEffectType => {
                "InvalidMasterSectionEffectType"
            }
            MasterSectionEffectType::Positive => "MasterSectionEffectTypePositive",
            MasterSectionEffectType::Negative => "MasterSectionEffectTypeNegative",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "InvalidMasterSectionEffectType" => {
                Some(Self::InvalidMasterSectionEffectType)
            }
            "MasterSectionEffectTypePositive" => Some(Self::Positive),
            "MasterSectionEffectTypeNegative" => Some(Self::Negative),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AttachType {
    InvalidAttachType = 0,
    MasterId = 1,
    Group = 2,
    Job = 3,
    Tag = 4,
}
impl AttachType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AttachType::InvalidAttachType => "InvalidAttachType",
            AttachType::MasterId => "MasterID",
            AttachType::Group => "Group",
            AttachType::Job => "Job",
            AttachType::Tag => "Tag",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "InvalidAttachType" => Some(Self::InvalidAttachType),
            "MasterID" => Some(Self::MasterId),
            "Group" => Some(Self::Group),
            "Job" => Some(Self::Job),
            "Tag" => Some(Self::Tag),
            _ => None,
        }
    }
}
