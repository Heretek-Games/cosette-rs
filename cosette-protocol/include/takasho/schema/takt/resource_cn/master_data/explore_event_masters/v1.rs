#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExploreEventMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub explore_event_master_id: i64,
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    #[prost(int64, tag = "4")]
    pub energy_cost: i64,
    #[prost(int64, tag = "5")]
    pub total_clock: i64,
    #[prost(int64, tag = "6")]
    pub lifetime_hours: i64,
    #[prost(
        enumeration = "super::super::super::super::super::score::resource_cn::common::v1::TriggerType",
        tag = "11"
    )]
    pub trigger_type: i32,
    #[prost(string, tag = "12")]
    pub trigger_param_1: ::prost::alloc::string::String,
    #[prost(string, tag = "13")]
    pub trigger_param_2: ::prost::alloc::string::String,
    #[prost(string, tag = "14")]
    pub trigger_param_3: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "21")]
    pub unlock_condition: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "22")]
    pub start_condition: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(enumeration = "ExploreEventType", tag = "23")]
    pub event_type: i32,
    #[prost(
        enumeration = "super::super::super::super::super::score::resource_cn::common::v1::LimitType",
        tag = "41"
    )]
    pub limit_type: i32,
    #[prost(int64, tag = "42")]
    pub limit_count: i64,
    #[prost(int64, tag = "43")]
    pub limit_cooldown_hours: i64,
    #[prost(string, repeated, tag = "51")]
    pub rewards: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "52")]
    pub bond_rewards: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, repeated, tag = "53")]
    pub display_character: ::prost::alloc::vec::Vec<i64>,
    #[prost(string, repeated, tag = "54")]
    pub first_time_rewards: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, tag = "61")]
    pub battle_character_master_id: i64,
    #[prost(bool, tag = "62")]
    pub need_bond_teabreak: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExploreEventMasters {
    #[prost(message, repeated, tag = "1")]
    pub explore_event_masters: ::prost::alloc::vec::Vec<ExploreEventMaster>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ExploreEventType {
    InvalidExploreType = 0,
    ExploreInvite = 1,
    ExplorePassiveInvite = 2,
    ExploreEntrust = 3,
    ExploreBox = 4,
    ExploreRiddle = 5,
    ExploreBattle = 6,
    ExploreDailyBox = 7,
}
impl ExploreEventType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ExploreEventType::InvalidExploreType => "INVALID_EXPLORE_TYPE",
            ExploreEventType::ExploreInvite => "EXPLORE_INVITE",
            ExploreEventType::ExplorePassiveInvite => "EXPLORE_PASSIVE_INVITE",
            ExploreEventType::ExploreEntrust => "EXPLORE_ENTRUST",
            ExploreEventType::ExploreBox => "EXPLORE_BOX",
            ExploreEventType::ExploreRiddle => "EXPLORE_RIDDLE",
            ExploreEventType::ExploreBattle => "EXPLORE_BATTLE",
            ExploreEventType::ExploreDailyBox => "EXPLORE_DAILY_BOX",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INVALID_EXPLORE_TYPE" => Some(Self::InvalidExploreType),
            "EXPLORE_INVITE" => Some(Self::ExploreInvite),
            "EXPLORE_PASSIVE_INVITE" => Some(Self::ExplorePassiveInvite),
            "EXPLORE_ENTRUST" => Some(Self::ExploreEntrust),
            "EXPLORE_BOX" => Some(Self::ExploreBox),
            "EXPLORE_RIDDLE" => Some(Self::ExploreRiddle),
            "EXPLORE_BATTLE" => Some(Self::ExploreBattle),
            "EXPLORE_DAILY_BOX" => Some(Self::ExploreDailyBox),
            _ => None,
        }
    }
}
