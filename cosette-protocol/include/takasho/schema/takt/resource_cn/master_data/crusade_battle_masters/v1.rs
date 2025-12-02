#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CrusadeBattleMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub crusade_battle_master_id: i64,
    #[prost(int64, tag = "3")]
    pub battle_stage_id: i64,
    #[prost(string, repeated, tag = "4")]
    pub first_cost: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "5")]
    pub cost: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "6")]
    pub first_rewards: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "7")]
    pub rewards: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(enumeration = "CrusadeBattleType", tag = "8")]
    pub r#type: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CrusadeBattleMasters {
    #[prost(message, repeated, tag = "1")]
    pub crusade_battle_masters: ::prost::alloc::vec::Vec<CrusadeBattleMaster>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CrusadeBattleType {
    Normal = 0,
    Resource = 1,
    Challenge = 2,
}
impl CrusadeBattleType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CrusadeBattleType::Normal => "NORMAL",
            CrusadeBattleType::Resource => "RESOURCE",
            CrusadeBattleType::Challenge => "CHALLENGE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NORMAL" => Some(Self::Normal),
            "RESOURCE" => Some(Self::Resource),
            "CHALLENGE" => Some(Self::Challenge),
            _ => None,
        }
    }
}
