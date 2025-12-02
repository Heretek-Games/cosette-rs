#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhonemeBuffMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub phoneme_buff_master_id: i64,
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    #[prost(int64, tag = "4")]
    pub phoneme_buff_group_master_id: i64,
    #[prost(int64, tag = "11")]
    pub weight: i64,
    #[prost(int64, tag = "12")]
    pub max_stack: i64,
    #[prost(int64, tag = "13")]
    pub priority: i64,
    #[prost(string, repeated, tag = "14")]
    pub attributes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, tag = "20")]
    pub attribute_id: i64,
    #[prost(string, tag = "30")]
    pub buff_skill: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhonemeBuffMasters {
    #[prost(message, repeated, tag = "1")]
    pub phoneme_buff_masters: ::prost::alloc::vec::Vec<PhonemeBuffMaster>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BuffType {
    Invalid = 0,
    Attribute = 1,
    BattleBuff = 2,
}
impl BuffType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            BuffType::Invalid => "INVALID",
            BuffType::Attribute => "ATTRIBUTE",
            BuffType::BattleBuff => "BATTLE_BUFF",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INVALID" => Some(Self::Invalid),
            "ATTRIBUTE" => Some(Self::Attribute),
            "BATTLE_BUFF" => Some(Self::BattleBuff),
            _ => None,
        }
    }
}
