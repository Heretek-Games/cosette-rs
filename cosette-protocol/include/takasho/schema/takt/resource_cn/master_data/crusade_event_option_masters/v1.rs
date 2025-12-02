#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CrusadeEventOptionMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub crusade_event_option_master_id: i64,
    #[prost(enumeration = "EventOptionChooseType", tag = "3")]
    pub choose: i32,
    #[prost(int64, tag = "4")]
    pub choose_value: i64,
    #[prost(enumeration = "EventOptionAppendType", tag = "5")]
    pub append_type: i32,
    #[prost(int64, repeated, tag = "6")]
    pub append_type_value: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, tag = "7")]
    pub append_value: i64,
    #[prost(enumeration = "EventOptionAppendType", tag = "8")]
    pub feedback_type: i32,
    #[prost(int64, repeated, tag = "9")]
    pub feedback_type_value: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, tag = "10")]
    pub feedback_value: i64,
    #[prost(int64, repeated, tag = "12")]
    pub success_results: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, repeated, tag = "13")]
    pub fail_results: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, tag = "14")]
    pub drop_master_id: i64,
    #[prost(int64, tag = "15")]
    pub success_adv: i64,
    #[prost(int64, tag = "16")]
    pub fail_adv: i64,
    #[prost(int64, tag = "17")]
    pub special_success_adv: i64,
    #[prost(int64, tag = "18")]
    pub special_fail_adv: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CrusadeEventOptionMasters {
    #[prost(message, repeated, tag = "1")]
    pub crusade_event_option_masters: ::prost::alloc::vec::Vec<CrusadeEventOptionMaster>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CrusadeEventOptionResultType {
    CrusadeEventOptionInvalid = 0,
    CrusadeEventOptionEnd = 1,
    CrusadeEventOptionDrop = 2,
    CrusadeEventOptionCamp = 3,
    CrusadeEventOptionReopen = 4,
    CrusadeEventOptionAllRecoverHp = 5,
    CrusadeEventOptionAllRecoverAp = 6,
    CrusadeEventOptionAllLossHp = 7,
    CrusadeEventOptionAllLossAp = 8,
    CrusadeEventOptionStartBattle = 9,
    CrusadeEventOptionLossAp = 10,
    CrusadeEventOptionLossHp = 11,
    CrusadeEventOptionRecoverHp = 12,
    CrusadeEventOptionRecoverAp = 13,
    CrusadeEventOptionReceive = 14,
    CrusadeEventOptionTreasure = 15,
    CrusadeEventOptionStartUndergroundEvent = 16,
}
impl CrusadeEventOptionResultType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CrusadeEventOptionResultType::CrusadeEventOptionInvalid => {
                "CrusadeEventOptionInvalid"
            }
            CrusadeEventOptionResultType::CrusadeEventOptionEnd => {
                "CrusadeEventOptionEnd"
            }
            CrusadeEventOptionResultType::CrusadeEventOptionDrop => {
                "CrusadeEventOptionDrop"
            }
            CrusadeEventOptionResultType::CrusadeEventOptionCamp => {
                "CrusadeEventOptionCamp"
            }
            CrusadeEventOptionResultType::CrusadeEventOptionReopen => {
                "CrusadeEventOptionReopen"
            }
            CrusadeEventOptionResultType::CrusadeEventOptionAllRecoverHp => {
                "CrusadeEventOptionAllRecoverHP"
            }
            CrusadeEventOptionResultType::CrusadeEventOptionAllRecoverAp => {
                "CrusadeEventOptionAllRecoverAP"
            }
            CrusadeEventOptionResultType::CrusadeEventOptionAllLossHp => {
                "CrusadeEventOptionAllLossHP"
            }
            CrusadeEventOptionResultType::CrusadeEventOptionAllLossAp => {
                "CrusadeEventOptionAllLossAP"
            }
            CrusadeEventOptionResultType::CrusadeEventOptionStartBattle => {
                "CrusadeEventOptionStartBattle"
            }
            CrusadeEventOptionResultType::CrusadeEventOptionLossAp => {
                "CrusadeEventOptionLossAP"
            }
            CrusadeEventOptionResultType::CrusadeEventOptionLossHp => {
                "CrusadeEventOptionLossHP"
            }
            CrusadeEventOptionResultType::CrusadeEventOptionRecoverHp => {
                "CrusadeEventOptionRecoverHP"
            }
            CrusadeEventOptionResultType::CrusadeEventOptionRecoverAp => {
                "CrusadeEventOptionRecoverAP"
            }
            CrusadeEventOptionResultType::CrusadeEventOptionReceive => {
                "CrusadeEventOptionReceive"
            }
            CrusadeEventOptionResultType::CrusadeEventOptionTreasure => {
                "CrusadeEventOptionTreasure"
            }
            CrusadeEventOptionResultType::CrusadeEventOptionStartUndergroundEvent => {
                "CrusadeEventOptionStartUndergroundEvent"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CrusadeEventOptionInvalid" => Some(Self::CrusadeEventOptionInvalid),
            "CrusadeEventOptionEnd" => Some(Self::CrusadeEventOptionEnd),
            "CrusadeEventOptionDrop" => Some(Self::CrusadeEventOptionDrop),
            "CrusadeEventOptionCamp" => Some(Self::CrusadeEventOptionCamp),
            "CrusadeEventOptionReopen" => Some(Self::CrusadeEventOptionReopen),
            "CrusadeEventOptionAllRecoverHP" => {
                Some(Self::CrusadeEventOptionAllRecoverHp)
            }
            "CrusadeEventOptionAllRecoverAP" => {
                Some(Self::CrusadeEventOptionAllRecoverAp)
            }
            "CrusadeEventOptionAllLossHP" => Some(Self::CrusadeEventOptionAllLossHp),
            "CrusadeEventOptionAllLossAP" => Some(Self::CrusadeEventOptionAllLossAp),
            "CrusadeEventOptionStartBattle" => Some(Self::CrusadeEventOptionStartBattle),
            "CrusadeEventOptionLossAP" => Some(Self::CrusadeEventOptionLossAp),
            "CrusadeEventOptionLossHP" => Some(Self::CrusadeEventOptionLossHp),
            "CrusadeEventOptionRecoverHP" => Some(Self::CrusadeEventOptionRecoverHp),
            "CrusadeEventOptionRecoverAP" => Some(Self::CrusadeEventOptionRecoverAp),
            "CrusadeEventOptionReceive" => Some(Self::CrusadeEventOptionReceive),
            "CrusadeEventOptionTreasure" => Some(Self::CrusadeEventOptionTreasure),
            "CrusadeEventOptionStartUndergroundEvent" => {
                Some(Self::CrusadeEventOptionStartUndergroundEvent)
            }
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EventOptionChooseType {
    Invalid = 0,
    CharacterWithHp = 1,
    CharacterNoHp = 2,
    CharacterWithAp = 3,
    CharacterNoAp = 4,
}
impl EventOptionChooseType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EventOptionChooseType::Invalid => "EventOptionChooseTypeInvalid",
            EventOptionChooseType::CharacterWithHp => {
                "EventOptionChooseTypeCharacterWithHp"
            }
            EventOptionChooseType::CharacterNoHp => "EventOptionChooseTypeCharacterNoHp",
            EventOptionChooseType::CharacterWithAp => {
                "EventOptionChooseTypeCharacterWithAp"
            }
            EventOptionChooseType::CharacterNoAp => "EventOptionChooseTypeCharacterNoAp",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EventOptionChooseTypeInvalid" => Some(Self::Invalid),
            "EventOptionChooseTypeCharacterWithHp" => Some(Self::CharacterWithHp),
            "EventOptionChooseTypeCharacterNoHp" => Some(Self::CharacterNoHp),
            "EventOptionChooseTypeCharacterWithAp" => Some(Self::CharacterWithAp),
            "EventOptionChooseTypeCharacterNoAp" => Some(Self::CharacterNoAp),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EventOptionAppendType {
    Invalid = 0,
    Character = 1,
    Camp = 2,
    Role = 3,
    Treasure = 4,
    Event = 5,
}
impl EventOptionAppendType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EventOptionAppendType::Invalid => "EventOptionAppendTypeInvalid",
            EventOptionAppendType::Character => "EventOptionAppendTypeCharacter",
            EventOptionAppendType::Camp => "EventOptionAppendTypeCamp",
            EventOptionAppendType::Role => "EventOptionAppendTypeRole",
            EventOptionAppendType::Treasure => "EventOptionAppendTypeTreasure",
            EventOptionAppendType::Event => "EventOptionAppendTypeEvent",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EventOptionAppendTypeInvalid" => Some(Self::Invalid),
            "EventOptionAppendTypeCharacter" => Some(Self::Character),
            "EventOptionAppendTypeCamp" => Some(Self::Camp),
            "EventOptionAppendTypeRole" => Some(Self::Role),
            "EventOptionAppendTypeTreasure" => Some(Self::Treasure),
            "EventOptionAppendTypeEvent" => Some(Self::Event),
            _ => None,
        }
    }
}
