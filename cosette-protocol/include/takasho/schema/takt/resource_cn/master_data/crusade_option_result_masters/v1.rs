#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CrusadeOptionResultMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub crusade_option_result_master_id: i64,
    #[prost(enumeration = "CrusadeOptionEffectTarget", tag = "3")]
    pub effective_object: i32,
    #[prost(enumeration = "CrusadeOptionEffectType", tag = "4")]
    pub effect_type: i32,
    #[prost(int64, tag = "5")]
    pub effect_value: i64,
    #[prost(int64, repeated, tag = "6")]
    pub treasure_random: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, tag = "8")]
    pub extra_drop: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CrusadeOptionResultMasters {
    #[prost(message, repeated, tag = "1")]
    pub crusade_option_result_masters: ::prost::alloc::vec::Vec<
        CrusadeOptionResultMaster,
    >,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CrusadeOptionEffectTarget {
    InvalidCrusadeOptionEffectTarget = 0,
    CrusadeOptionEffectToChosen = 1,
    CrusadeOptionEffectToFriendPartyRandom = 2,
    CrusadeOptionEffectToFriendParty = 3,
}
impl CrusadeOptionEffectTarget {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CrusadeOptionEffectTarget::InvalidCrusadeOptionEffectTarget => {
                "InvalidCrusadeOptionEffectTarget"
            }
            CrusadeOptionEffectTarget::CrusadeOptionEffectToChosen => {
                "CrusadeOptionEffectToChosen"
            }
            CrusadeOptionEffectTarget::CrusadeOptionEffectToFriendPartyRandom => {
                "CrusadeOptionEffectToFriendPartyRandom"
            }
            CrusadeOptionEffectTarget::CrusadeOptionEffectToFriendParty => {
                "CrusadeOptionEffectToFriendParty"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "InvalidCrusadeOptionEffectTarget" => {
                Some(Self::InvalidCrusadeOptionEffectTarget)
            }
            "CrusadeOptionEffectToChosen" => Some(Self::CrusadeOptionEffectToChosen),
            "CrusadeOptionEffectToFriendPartyRandom" => {
                Some(Self::CrusadeOptionEffectToFriendPartyRandom)
            }
            "CrusadeOptionEffectToFriendParty" => {
                Some(Self::CrusadeOptionEffectToFriendParty)
            }
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CrusadeOptionEffectType {
    InvalidCrusadeOptionEffectType = 0,
    CrusadeOptionRecoverHp = 1,
    CrusadeOptionLossHp = 2,
    CrusadeOptionReopen = 3,
    CrusadeOptionRecoverAp = 4,
    CrusadeOptionLossAp = 5,
}
impl CrusadeOptionEffectType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CrusadeOptionEffectType::InvalidCrusadeOptionEffectType => {
                "InvalidCrusadeOptionEffectType"
            }
            CrusadeOptionEffectType::CrusadeOptionRecoverHp => "CrusadeOptionRecoverHp",
            CrusadeOptionEffectType::CrusadeOptionLossHp => "CrusadeOptionLossHp",
            CrusadeOptionEffectType::CrusadeOptionReopen => "CrusadeOptionReopen",
            CrusadeOptionEffectType::CrusadeOptionRecoverAp => "CrusadeOptionRecoverAp",
            CrusadeOptionEffectType::CrusadeOptionLossAp => "CrusadeOptionLossAp",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "InvalidCrusadeOptionEffectType" => {
                Some(Self::InvalidCrusadeOptionEffectType)
            }
            "CrusadeOptionRecoverHp" => Some(Self::CrusadeOptionRecoverHp),
            "CrusadeOptionLossHp" => Some(Self::CrusadeOptionLossHp),
            "CrusadeOptionReopen" => Some(Self::CrusadeOptionReopen),
            "CrusadeOptionRecoverAp" => Some(Self::CrusadeOptionRecoverAp),
            "CrusadeOptionLossAp" => Some(Self::CrusadeOptionLossAp),
            _ => None,
        }
    }
}
