#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerState {
    #[prost(string, tag = "1")]
    pub player_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub idp_account_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub session_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "4")]
    pub realm_id: i64,
    #[prost(int64, tag = "5")]
    pub lv: i64,
    #[prost(int64, tag = "6")]
    pub paid_gem: i64,
    #[prost(int64, tag = "7")]
    pub free_gem: i64,
    #[prost(int64, tag = "8")]
    pub gold: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ARst {
    #[prost(string, tag = "1")]
    pub obj: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub before: i64,
    #[prost(int64, tag = "3")]
    pub after: i64,
    #[prost(int64, tag = "4")]
    pub diff: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Empty {}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoleLoginV1Payload {
    #[prost(message, optional, tag = "1")]
    pub state: ::core::option::Option<PlayerState>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoleCreateV1Payload {
    #[prost(message, repeated, tag = "1")]
    pub a_rst: ::prost::alloc::vec::Vec<ARst>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoleGuideV1Payload {
    #[prost(int64, tag = "1")]
    pub guide_id: i64,
    #[prost(message, repeated, tag = "2")]
    pub a_rst: ::prost::alloc::vec::Vec<ARst>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoleNameV1Payload {
    #[prost(string, tag = "1")]
    pub new_name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub a_rst: ::prost::alloc::vec::Vec<ARst>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ItemUseV1Payload {
    #[prost(int64, tag = "1")]
    pub item_id: i64,
    #[prost(int64, tag = "2")]
    pub num: i64,
    #[prost(int64, repeated, tag = "3")]
    pub position: ::prost::alloc::vec::Vec<i64>,
    #[prost(message, repeated, tag = "4")]
    pub a_rst: ::prost::alloc::vec::Vec<ARst>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ItemSellV1Payload {
    #[prost(int64, tag = "1")]
    pub item_id: i64,
    #[prost(int64, tag = "2")]
    pub num: i64,
    #[prost(message, repeated, tag = "4")]
    pub a_rst: ::prost::alloc::vec::Vec<ARst>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CardLevelUpV1Payload {
    #[prost(int64, tag = "1")]
    pub card_id: i64,
    #[prost(int64, tag = "2")]
    pub card_level: i64,
    #[prost(message, repeated, tag = "3")]
    pub a_rst: ::prost::alloc::vec::Vec<ARst>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CardGradeUpV1Payload {
    #[prost(int64, tag = "1")]
    pub card_id: i64,
    #[prost(int64, tag = "2")]
    pub card_grade: i64,
    #[prost(int64, tag = "3")]
    pub panel_id: i64,
    #[prost(message, repeated, tag = "4")]
    pub a_rst: ::prost::alloc::vec::Vec<ARst>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CardStarUpV1Payload {
    #[prost(int64, tag = "1")]
    pub card_id: i64,
    #[prost(int64, tag = "2")]
    pub card_star: i64,
    #[prost(message, repeated, tag = "3")]
    pub a_rst: ::prost::alloc::vec::Vec<ARst>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EquipFitV1Payload {
    #[prost(int64, tag = "1")]
    pub equip_id: i64,
    #[prost(int64, tag = "2")]
    pub card_id: i64,
    #[prost(enumeration = "TypeEquipmentFit", tag = "3")]
    pub r#type: i32,
    #[prost(int64, tag = "4")]
    pub position: i64,
    #[prost(message, repeated, tag = "5")]
    pub a_rst: ::prost::alloc::vec::Vec<ARst>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EquipLevelUpV1Payload {
    #[prost(int64, tag = "1")]
    pub equip_id: i64,
    #[prost(int64, tag = "2")]
    pub equip_level: i64,
    #[prost(message, repeated, tag = "3")]
    pub a_rst: ::prost::alloc::vec::Vec<ARst>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EquipGradeUpV1Payload {
    #[prost(int64, tag = "1")]
    pub equip_id: i64,
    #[prost(int64, tag = "2")]
    pub equip_grade: i64,
    #[prost(message, repeated, tag = "3")]
    pub a_rst: ::prost::alloc::vec::Vec<ARst>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EquipStarUpV1Payload {
    #[prost(int64, tag = "1")]
    pub equip_id: i64,
    #[prost(int64, tag = "2")]
    pub equip_star: i64,
    #[prost(message, repeated, tag = "3")]
    pub a_rst: ::prost::alloc::vec::Vec<ARst>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PveSectionUpdateV1Payload {
    #[prost(int64, tag = "1")]
    pub stage_id: i64,
    #[prost(string, tag = "2")]
    pub flag: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    pub time: i64,
    #[prost(message, repeated, tag = "4")]
    pub a_rst: ::prost::alloc::vec::Vec<ARst>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PveSectionFinishV1Payload {
    #[prost(int64, tag = "1")]
    pub stage_id: i64,
    #[prost(message, repeated, tag = "2")]
    pub a_rst: ::prost::alloc::vec::Vec<ARst>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PveBattleFinishV1Payload {
    #[prost(int64, tag = "1")]
    pub stage_id: i64,
    #[prost(int64, tag = "2")]
    pub battle_id: i64,
    #[prost(enumeration = "TypeEventStatus", tag = "3")]
    pub result: i32,
    #[prost(int64, repeated, tag = "4")]
    pub team: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, tag = "5")]
    pub time: i64,
    #[prost(message, repeated, tag = "6")]
    pub a_rst: ::prost::alloc::vec::Vec<ARst>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PveSectionStartV1Payload {
    #[prost(int64, tag = "1")]
    pub stage_id: i64,
    #[prost(int64, tag = "2")]
    pub battle_id: i64,
    #[prost(enumeration = "TypeStageEnter", tag = "3")]
    pub r#type: i32,
    #[prost(message, repeated, tag = "4")]
    pub a_rst: ::prost::alloc::vec::Vec<ARst>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UseKeywordV1Payload {
    #[prost(int64, tag = "1")]
    pub card_id: i64,
    #[prost(int64, tag = "2")]
    pub keyword: i64,
    #[prost(message, repeated, tag = "4")]
    pub a_rst: ::prost::alloc::vec::Vec<ARst>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExploreUnlockEventV1Payload {
    #[prost(int64, tag = "1")]
    pub explore_id: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExploreStartEventV1Payload {
    #[prost(int64, tag = "1")]
    pub explore_id: i64,
    #[prost(message, repeated, tag = "2")]
    pub a_rst: ::prost::alloc::vec::Vec<ARst>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExploreFinishEventAdvV1Payload {
    #[prost(int64, tag = "1")]
    pub explore_id: i64,
    #[prost(int64, tag = "2")]
    pub adv_id: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExploreFinishEventV1Payload {
    #[prost(int64, tag = "1")]
    pub explore_id: i64,
    #[prost(int64, tag = "2")]
    pub is_partial: i64,
    #[prost(message, repeated, tag = "3")]
    pub a_rst: ::prost::alloc::vec::Vec<ARst>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExploreGiftV1Payload {
    #[prost(int64, tag = "1")]
    pub card_id: i64,
    #[prost(int64, tag = "2")]
    pub item_id: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EventId {
    Invalid = 0,
    RoleLogin = 1,
    RoleCreate = 2,
    RoleGuide = 3,
    RoleName = 4,
    ItemUse = 11,
    ItemSell = 12,
    CardLevelup = 21,
    CardGradeup = 22,
    CardStarup = 23,
    EquipUse = 31,
    EquipLevelup = 32,
    EquipGradeup = 33,
    EquipStarup = 34,
    StageFinish = 41,
    StagePlot = 42,
    StageFight = 43,
    StageEnter = 44,
    TaskReward = 51,
    ExploreKeyword = 61,
    ExploreUnlockEvent = 62,
    ExploreStartEvent = 63,
    ExploreFinishEventAdv = 64,
    ExploreFinishEvent = 65,
    ExploreGift = 66,
}
impl EventId {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EventId::Invalid => "Invalid",
            EventId::RoleLogin => "role_login",
            EventId::RoleCreate => "role_create",
            EventId::RoleGuide => "role_guide",
            EventId::RoleName => "role_name",
            EventId::ItemUse => "item_use",
            EventId::ItemSell => "item_sell",
            EventId::CardLevelup => "card_levelup",
            EventId::CardGradeup => "card_gradeup",
            EventId::CardStarup => "card_starup",
            EventId::EquipUse => "equip_use",
            EventId::EquipLevelup => "equip_levelup",
            EventId::EquipGradeup => "equip_gradeup",
            EventId::EquipStarup => "equip_starup",
            EventId::StageFinish => "stage_finish",
            EventId::StagePlot => "stage_plot",
            EventId::StageFight => "stage_fight",
            EventId::StageEnter => "stage_enter",
            EventId::TaskReward => "task_reward",
            EventId::ExploreKeyword => "explore_keyword",
            EventId::ExploreUnlockEvent => "explore_unlock_event",
            EventId::ExploreStartEvent => "explore_start_event",
            EventId::ExploreFinishEventAdv => "explore_finish_event_adv",
            EventId::ExploreFinishEvent => "explore_finish_event",
            EventId::ExploreGift => "explore_gift",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Invalid" => Some(Self::Invalid),
            "role_login" => Some(Self::RoleLogin),
            "role_create" => Some(Self::RoleCreate),
            "role_guide" => Some(Self::RoleGuide),
            "role_name" => Some(Self::RoleName),
            "item_use" => Some(Self::ItemUse),
            "item_sell" => Some(Self::ItemSell),
            "card_levelup" => Some(Self::CardLevelup),
            "card_gradeup" => Some(Self::CardGradeup),
            "card_starup" => Some(Self::CardStarup),
            "equip_use" => Some(Self::EquipUse),
            "equip_levelup" => Some(Self::EquipLevelup),
            "equip_gradeup" => Some(Self::EquipGradeup),
            "equip_starup" => Some(Self::EquipStarup),
            "stage_finish" => Some(Self::StageFinish),
            "stage_plot" => Some(Self::StagePlot),
            "stage_fight" => Some(Self::StageFight),
            "stage_enter" => Some(Self::StageEnter),
            "task_reward" => Some(Self::TaskReward),
            "explore_keyword" => Some(Self::ExploreKeyword),
            "explore_unlock_event" => Some(Self::ExploreUnlockEvent),
            "explore_start_event" => Some(Self::ExploreStartEvent),
            "explore_finish_event_adv" => Some(Self::ExploreFinishEventAdv),
            "explore_finish_event" => Some(Self::ExploreFinishEvent),
            "explore_gift" => Some(Self::ExploreGift),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TypeEquipmentFit {
    InvalidEquipmentFitType = 0,
    Fit = 1,
    UnFit = 2,
}
impl TypeEquipmentFit {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TypeEquipmentFit::InvalidEquipmentFitType => "InvalidEquipmentFitType",
            TypeEquipmentFit::Fit => "Fit",
            TypeEquipmentFit::UnFit => "UnFit",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "InvalidEquipmentFitType" => Some(Self::InvalidEquipmentFitType),
            "Fit" => Some(Self::Fit),
            "UnFit" => Some(Self::UnFit),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TypeEventStatus {
    InvalidEventStatus = 0,
    Success = 1,
    Failed = 2,
}
impl TypeEventStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TypeEventStatus::InvalidEventStatus => "InvalidEventStatus",
            TypeEventStatus::Success => "Success",
            TypeEventStatus::Failed => "Failed",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "InvalidEventStatus" => Some(Self::InvalidEventStatus),
            "Success" => Some(Self::Success),
            "Failed" => Some(Self::Failed),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TypeStageEnter {
    InvalidStageEnterType = 0,
    Section = 1,
    Battle = 2,
}
impl TypeStageEnter {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TypeStageEnter::InvalidStageEnterType => "InvalidStageEnterType",
            TypeStageEnter::Section => "Section",
            TypeStageEnter::Battle => "Battle",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "InvalidStageEnterType" => Some(Self::InvalidStageEnterType),
            "Section" => Some(Self::Section),
            "Battle" => Some(Self::Battle),
            _ => None,
        }
    }
}
