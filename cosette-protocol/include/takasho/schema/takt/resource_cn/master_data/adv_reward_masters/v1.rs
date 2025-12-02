#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdvRewardMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub adv_reward_master_id: i64,
    #[prost(int64, tag = "3")]
    pub chapter: i64,
    #[prost(int64, tag = "4")]
    pub section: i64,
    #[prost(string, tag = "5")]
    pub flag: ::prost::alloc::string::String,
    #[prost(int64, tag = "6")]
    pub battle_character_master_id: i64,
    #[prost(int64, tag = "7")]
    pub value: i64,
    #[prost(enumeration = "RewardType", tag = "8")]
    pub many_times: i32,
    #[prost(string, repeated, tag = "9")]
    pub rewards: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdvRewardMasters {
    #[prost(message, repeated, tag = "1")]
    pub adv_reward_masters: ::prost::alloc::vec::Vec<AdvRewardMaster>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RewardType {
    OneTime = 0,
    Repeated = 1,
}
impl RewardType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RewardType::OneTime => "ONE_TIME",
            RewardType::Repeated => "REPEATED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ONE_TIME" => Some(Self::OneTime),
            "REPEATED" => Some(Self::Repeated),
            _ => None,
        }
    }
}
