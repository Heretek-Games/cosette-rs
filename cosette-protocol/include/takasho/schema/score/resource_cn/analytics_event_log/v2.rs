#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoleInfo {
    #[prost(string, tag = "1")]
    pub role_key: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub role_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub zone_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub level: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub vip: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub player_id: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub role_name: ::prost::alloc::string::String,
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
pub struct BaseInfo {
    #[prost(sint64, tag = "1")]
    pub last_login_day: i64,
    #[prost(sint64, tag = "2")]
    pub create_day: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CoinInfo {
    #[prost(int64, tag = "1")]
    pub diamond: i64,
    #[prost(int64, tag = "2")]
    pub free_diamond: i64,
    #[prost(int64, tag = "3")]
    pub stone: i64,
    #[prost(int64, tag = "4")]
    pub free_stone: i64,
    #[prost(int64, tag = "5")]
    pub gold: i64,
    #[prost(int64, tag = "6")]
    pub strength: i64,
    #[prost(int64, tag = "7")]
    pub exp: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StageInfo {
    #[prost(int64, tag = "1")]
    pub normal_stage: i64,
    #[prost(int64, tag = "2")]
    pub hard_stage: i64,
    #[prost(int64, tag = "3")]
    pub guide: i64,
    #[prost(message, repeated, tag = "4")]
    pub dq_stage_info: ::prost::alloc::vec::Vec<DqStageInfo>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DqStageInfo {
    #[prost(int64, tag = "1")]
    pub dq_chapter_id: i64,
    #[prost(int64, tag = "2")]
    pub dq_id: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CardInfo {
    #[prost(int64, tag = "1")]
    pub level: i64,
    #[prost(int64, tag = "2")]
    pub star: i64,
    #[prost(int64, tag = "3")]
    pub grade: i64,
    #[prost(int64, repeated, tag = "4")]
    pub skill: ::prost::alloc::vec::Vec<i64>,
    #[prost(string, tag = "5")]
    pub arrange: ::prost::alloc::string::String,
    #[prost(int64, repeated, tag = "6")]
    pub sub_star: ::prost::alloc::vec::Vec<i64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EquipInfo {
    #[prost(int64, tag = "1")]
    pub level: i64,
    #[prost(int64, tag = "2")]
    pub star: i64,
    #[prost(int64, tag = "3")]
    pub grade: i64,
    #[prost(int64, tag = "4")]
    pub card_id: i64,
    #[prost(int64, tag = "5")]
    pub keyword: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerEquipInfo {
    #[prost(int64, tag = "1")]
    pub equip_id: i64,
    #[prost(string, tag = "2")]
    pub unique_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub equip_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub equip_info: ::core::option::Option<EquipInfo>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhonemeInfo {
    #[prost(int64, tag = "1")]
    pub level: i64,
    #[prost(int64, tag = "2")]
    pub slot: i64,
    #[prost(int64, tag = "3")]
    pub card_id: i64,
    #[prost(message, repeated, tag = "4")]
    pub buff_values: ::prost::alloc::vec::Vec<phoneme_info::BuffValue>,
}
/// Nested message and enum types in `PhonemeInfo`.
pub mod phoneme_info {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BuffValue {
        #[prost(int64, tag = "1")]
        pub buff_id: i64,
        #[prost(int64, repeated, tag = "2")]
        pub value: ::prost::alloc::vec::Vec<i64>,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerPhonemeInfo {
    #[prost(int64, tag = "1")]
    pub phoneme_id: i64,
    #[prost(string, tag = "2")]
    pub unique_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub phoneme_info: ::core::option::Option<PhonemeInfo>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FightInfo {
    #[prost(int64, tag = "1")]
    pub round: i64,
    #[prost(int64, repeated, tag = "2")]
    pub dead_card: ::prost::alloc::vec::Vec<i64>,
    #[prost(message, repeated, tag = "3")]
    pub damage: ::prost::alloc::vec::Vec<fight_info::DamageEntry>,
    #[prost(message, repeated, tag = "4")]
    pub skill_use: ::prost::alloc::vec::Vec<fight_info::SkillUseEntry>,
    #[prost(bool, repeated, tag = "5")]
    pub condition: ::prost::alloc::vec::Vec<bool>,
    #[prost(string, tag = "6")]
    pub auto: ::prost::alloc::string::String,
    #[prost(int64, repeated, tag = "7")]
    pub monster: ::prost::alloc::vec::Vec<i64>,
}
/// Nested message and enum types in `FightInfo`.
pub mod fight_info {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DamageEntry {
        #[prost(int64, tag = "1")]
        pub key: i64,
        #[prost(int64, tag = "2")]
        pub value: i64,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SkillUseEntry {
        #[prost(int64, tag = "1")]
        pub key: i64,
        #[prost(int64, tag = "2")]
        pub value: i64,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FightResultInfo {
    #[prost(string, tag = "1")]
    pub successful: ::prost::alloc::string::String,
    #[prost(int64, repeated, tag = "2")]
    pub achievements: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, repeated, tag = "3")]
    pub stars: ::prost::alloc::vec::Vec<i64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ItemInfo {
    #[prost(int64, tag = "1")]
    pub item_id: i64,
    #[prost(int64, tag = "2")]
    pub item_count: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Team {
    #[prost(int64, repeated, tag = "1")]
    pub battle_character_ids: ::prost::alloc::vec::Vec<i64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlotInfo {}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExploreInfo {}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtraInfo {}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GoodsInfo {
    #[prost(int64, tag = "1")]
    pub item_id: i64,
    #[prost(string, tag = "2")]
    pub item_name: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    pub position: i64,
    #[prost(string, tag = "4")]
    pub discount: ::prost::alloc::string::String,
    #[prost(int64, tag = "5")]
    pub category: i64,
    #[prost(int64, tag = "6")]
    pub num: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BadgeInfo {
    #[prost(int64, tag = "1")]
    pub badge_id: i64,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerRoleSnapshotPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_role_snapshot_payload::ServerRoleSnapshotBody,
    >,
}
/// Nested message and enum types in `ServerRoleSnapshotPayload`.
pub mod server_role_snapshot_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerRoleSnapshotBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_role_snapshot_body::ServerRoleSnapshotATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerRoleSnapshotBody`.
    pub mod server_role_snapshot_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerRoleSnapshotATar {
            #[prost(string, tag = "1")]
            pub role_name: ::prost::alloc::string::String,
            #[prost(message, optional, tag = "2")]
            pub base_info: ::core::option::Option<super::super::BaseInfo>,
            #[prost(message, optional, tag = "3")]
            pub coin_info: ::core::option::Option<super::super::CoinInfo>,
            #[prost(message, optional, tag = "4")]
            pub stage_info: ::core::option::Option<super::super::StageInfo>,
            #[prost(int64, tag = "5")]
            pub avatar_id: i64,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerRoleCreatePayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<server_role_create_payload::ServerRoleCreateBody>,
}
/// Nested message and enum types in `ServerRoleCreatePayload`.
pub mod server_role_create_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerRoleCreateBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<server_role_create_body::ServerRoleCreateATar>,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerRoleCreateBody`.
    pub mod server_role_create_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerRoleCreateATar {
            #[prost(string, tag = "1")]
            pub role_name: ::prost::alloc::string::String,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerRoleLoginPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<server_role_login_payload::ServerRoleLoginBody>,
}
/// Nested message and enum types in `ServerRoleLoginPayload`.
pub mod server_role_login_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerRoleLoginBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<server_role_login_body::ServerRoleLoginATar>,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerRoleLoginBody`.
    pub mod server_role_login_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerRoleLoginATar {
            #[prost(enumeration = "super::LoginType", tag = "1")]
            pub r#type: i32,
        }
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum LoginType {
        InvalidLoginType = 0,
        NormalLoginType = 1,
        ReconnectLoginType = 2,
    }
    impl LoginType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                LoginType::InvalidLoginType => "InvalidLoginType",
                LoginType::NormalLoginType => "NormalLoginType",
                LoginType::ReconnectLoginType => "ReconnectLoginType",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "InvalidLoginType" => Some(Self::InvalidLoginType),
                "NormalLoginType" => Some(Self::NormalLoginType),
                "ReconnectLoginType" => Some(Self::ReconnectLoginType),
                _ => None,
            }
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerRoleGuidePayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<server_role_guide_payload::ServerRoleGuideBody>,
}
/// Nested message and enum types in `ServerRoleGuidePayload`.
pub mod server_role_guide_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerRoleGuideBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<server_role_guide_body::ServerRoleGuideATar>,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerRoleGuideBody`.
    pub mod server_role_guide_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerRoleGuideATar {
            #[prost(int64, tag = "1")]
            pub guide_id: i64,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerRoleLevelUpPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_role_level_up_payload::ServerRoleLevelUpBody,
    >,
}
/// Nested message and enum types in `ServerRoleLevelUpPayload`.
pub mod server_role_level_up_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerRoleLevelUpBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_role_level_up_body::ServerRoleLevelUpATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerRoleLevelUpBody`.
    pub mod server_role_level_up_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerRoleLevelUpATar {
            #[prost(int64, tag = "1")]
            pub level_before: i64,
            #[prost(int64, tag = "2")]
            pub level_after: i64,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerRoleRenamePayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<server_role_rename_payload::ServerRoleRenameBody>,
}
/// Nested message and enum types in `ServerRoleRenamePayload`.
pub mod server_role_rename_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerRoleRenameBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<server_role_rename_body::ServerRoleRenameATar>,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerRoleRenameBody`.
    pub mod server_role_rename_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerRoleRenameATar {
            #[prost(string, tag = "1")]
            pub name_before: ::prost::alloc::string::String,
            #[prost(string, tag = "2")]
            pub name_after: ::prost::alloc::string::String,
            #[prost(enumeration = "super::RoleRenameType", tag = "3")]
            pub r#type: i32,
            #[prost(enumeration = "super::RoleRenameFee", tag = "4")]
            pub is_free: i32,
        }
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum RoleRenameType {
        InvalidRenameType = 0,
        SetNameFirstTime = 1,
        ChangeName = 2,
    }
    impl RoleRenameType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RoleRenameType::InvalidRenameType => "InvalidRenameType",
                RoleRenameType::SetNameFirstTime => "SetNameFirstTime",
                RoleRenameType::ChangeName => "ChangeName",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "InvalidRenameType" => Some(Self::InvalidRenameType),
                "SetNameFirstTime" => Some(Self::SetNameFirstTime),
                "ChangeName" => Some(Self::ChangeName),
                _ => None,
            }
        }
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum RoleRenameFee {
        InvalidRenameFee = 0,
        PaidRename = 1,
        RenameForFree = 2,
    }
    impl RoleRenameFee {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RoleRenameFee::InvalidRenameFee => "InvalidRenameFee",
                RoleRenameFee::PaidRename => "PaidRename",
                RoleRenameFee::RenameForFree => "RenameForFree",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "InvalidRenameFee" => Some(Self::InvalidRenameFee),
                "PaidRename" => Some(Self::PaidRename),
                "RenameForFree" => Some(Self::RenameForFree),
                _ => None,
            }
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerRefreshTargetListPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_refresh_target_list_payload::ServerRefreshTargetListBody,
    >,
}
/// Nested message and enum types in `ServerRefreshTargetListPayload`.
pub mod server_refresh_target_list_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerRefreshTargetListBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_refresh_target_list_body::ServerRefreshTargetListATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerRefreshTargetListBody`.
    pub mod server_refresh_target_list_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerRefreshTargetListATar {
            #[prost(int64, repeated, tag = "1")]
            pub finish_target_ids: ::prost::alloc::vec::Vec<i64>,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerUnlockTargetsPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_unlock_targets_payload::ServerUnlockTargetsBody,
    >,
}
/// Nested message and enum types in `ServerUnlockTargetsPayload`.
pub mod server_unlock_targets_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerUnlockTargetsBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_unlock_targets_body::ServerUnlockTargetsATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerUnlockTargetsBody`.
    pub mod server_unlock_targets_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerUnlockTargetsATar {
            #[prost(int64, repeated, tag = "1")]
            pub target_ids: ::prost::alloc::vec::Vec<i64>,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerFinishTargetsPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_finish_targets_payload::ServerFinishTargetsBody,
    >,
}
/// Nested message and enum types in `ServerFinishTargetsPayload`.
pub mod server_finish_targets_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerFinishTargetsBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_finish_targets_body::ServerFinishTargetsATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerFinishTargetsBody`.
    pub mod server_finish_targets_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerFinishTargetsATar {
            #[prost(int64, repeated, tag = "1")]
            pub target_ids: ::prost::alloc::vec::Vec<i64>,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerFunctionUnlockPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_function_unlock_payload::ServerFunctionUnlockBody,
    >,
}
/// Nested message and enum types in `ServerFunctionUnlockPayload`.
pub mod server_function_unlock_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerFunctionUnlockBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_function_unlock_body::ServerFunctionUnlockATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerFunctionUnlockBody`.
    pub mod server_function_unlock_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerFunctionUnlockATar {
            #[prost(int64, tag = "1")]
            pub function_code: i64,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerMonthlyCheckinPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_monthly_checkin_payload::ServerMonthlyCheckinBody,
    >,
}
/// Nested message and enum types in `ServerMonthlyCheckinPayload`.
pub mod server_monthly_checkin_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerMonthlyCheckinBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_monthly_checkin_body::ServerMonthlyCheckinATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerMonthlyCheckinBody`.
    pub mod server_monthly_checkin_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerMonthlyCheckinATar {
            #[prost(int64, tag = "1")]
            pub day_count: i64,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerGiftCodeRedeemPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_gift_code_redeem_payload::ServerGiftCodeRedeemBody,
    >,
}
/// Nested message and enum types in `ServerGiftCodeRedeemPayload`.
pub mod server_gift_code_redeem_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerGiftCodeRedeemBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_gift_code_redeem_body::ServerGiftCodeRedeemATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerGiftCodeRedeemBody`.
    pub mod server_gift_code_redeem_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerGiftCodeRedeemATar {
            #[prost(string, tag = "1")]
            pub gift_code: ::prost::alloc::string::String,
            #[prost(string, tag = "2")]
            pub batch_code: ::prost::alloc::string::String,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerRoleDeletePayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<server_role_delete_payload::ServerRoleDeleteBody>,
}
/// Nested message and enum types in `ServerRoleDeletePayload`.
pub mod server_role_delete_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerRoleDeleteBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<server_role_delete_body::ServerRoleDeleteATar>,
        #[prost(message, optional, tag = "2")]
        pub a_rst: ::core::option::Option<super::ARst>,
    }
    /// Nested message and enum types in `ServerRoleDeleteBody`.
    pub mod server_role_delete_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerRoleDeleteATar {
            #[prost(int64, repeated, tag = "1")]
            pub card_ids: ::prost::alloc::vec::Vec<i64>,
            #[prost(int64, repeated, tag = "2")]
            pub equip_ids: ::prost::alloc::vec::Vec<i64>,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerGameSharePayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<server_game_share_payload::ServerGameShareBody>,
}
/// Nested message and enum types in `ServerGameSharePayload`.
pub mod server_game_share_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerGameShareBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<server_game_share_body::ServerGameShareATar>,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerGameShareBody`.
    pub mod server_game_share_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerGameShareATar {
            #[prost(string, tag = "1")]
            pub is_reward: ::prost::alloc::string::String,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerRoleSetAvatarPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_role_set_avatar_payload::ServerRoleSetAvatarBody,
    >,
}
/// Nested message and enum types in `ServerRoleSetAvatarPayload`.
pub mod server_role_set_avatar_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerRoleSetAvatarBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_role_set_avatar_body::ServerRoleSetAvatarATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerRoleSetAvatarBody`.
    pub mod server_role_set_avatar_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerRoleSetAvatarATar {
            #[prost(int64, tag = "1")]
            pub avatar_before: i64,
            #[prost(int64, tag = "2")]
            pub avatar_after: i64,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerRoleFrameChangePayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_role_frame_change_payload::ServerRoleFrameChangeBody,
    >,
}
/// Nested message and enum types in `ServerRoleFrameChangePayload`.
pub mod server_role_frame_change_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerRoleFrameChangeBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_role_frame_change_body::ServerRoleFrameChangeATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerRoleFrameChangeBody`.
    pub mod server_role_frame_change_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerRoleFrameChangeATar {
            #[prost(int64, tag = "1")]
            pub frame_before: i64,
            #[prost(int64, tag = "2")]
            pub frame_after: i64,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerStageEnterPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<server_stage_enter_payload::ServerStageEnterBody>,
}
/// Nested message and enum types in `ServerStageEnterPayload`.
pub mod server_stage_enter_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerStageEnterBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<server_stage_enter_body::ServerStageEnterATar>,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerStageEnterBody`.
    pub mod server_stage_enter_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerStageEnterATar {
            #[prost(int64, tag = "1")]
            pub stage_id: i64,
            #[prost(string, tag = "2")]
            pub stage_name: ::prost::alloc::string::String,
            #[prost(int64, tag = "3")]
            pub battle_id: i64,
            #[prost(enumeration = "super::PveType", tag = "4")]
            pub r#type: i32,
            #[prost(int64, tag = "5")]
            pub stamina_cost: i64,
        }
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum PveType {
        InvalidPveType = 0,
        ExplorePveType = 1,
        BattlePveType = 2,
        InterPveType = 3,
        PlotPveType = 4,
    }
    impl PveType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PveType::InvalidPveType => "InvalidPveType",
                PveType::ExplorePveType => "ExplorePveType",
                PveType::BattlePveType => "BattlePveType",
                PveType::InterPveType => "InterPveType",
                PveType::PlotPveType => "PlotPveType",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "InvalidPveType" => Some(Self::InvalidPveType),
                "ExplorePveType" => Some(Self::ExplorePveType),
                "BattlePveType" => Some(Self::BattlePveType),
                "InterPveType" => Some(Self::InterPveType),
                "PlotPveType" => Some(Self::PlotPveType),
                _ => None,
            }
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerStageFightPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<server_stage_fight_payload::ServerStageFightBody>,
}
/// Nested message and enum types in `ServerStageFightPayload`.
pub mod server_stage_fight_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerStageFightBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<server_stage_fight_body::ServerStageFightATar>,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerStageFightBody`.
    pub mod server_stage_fight_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerStageFightATar {
            #[prost(int64, tag = "1")]
            pub stage_id: i64,
            #[prost(string, tag = "2")]
            pub stage_name: ::prost::alloc::string::String,
            #[prost(int64, tag = "3")]
            pub battle_id: i64,
            #[prost(message, optional, tag = "4")]
            pub team: ::core::option::Option<super::super::Team>,
            #[prost(enumeration = "super::PveResult", tag = "5")]
            pub result: i32,
            #[prost(int64, tag = "6")]
            pub time: i64,
            #[prost(message, repeated, tag = "7")]
            pub card_info: ::prost::alloc::vec::Vec<super::super::CardInfo>,
            #[prost(message, optional, tag = "8")]
            pub fight_info: ::core::option::Option<super::super::FightInfo>,
            #[prost(message, optional, tag = "9")]
            pub result_info: ::core::option::Option<super::super::FightResultInfo>,
            #[prost(message, repeated, tag = "10")]
            pub equip_info: ::prost::alloc::vec::Vec<super::super::EquipInfo>,
        }
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum PveResult {
        InvalidPveResult = 0,
        Success = 1,
        Fail = 2,
    }
    impl PveResult {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PveResult::InvalidPveResult => "InvalidPveResult",
                PveResult::Success => "PveResultSuccess",
                PveResult::Fail => "PveResultFail",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "InvalidPveResult" => Some(Self::InvalidPveResult),
                "PveResultSuccess" => Some(Self::Success),
                "PveResultFail" => Some(Self::Fail),
                _ => None,
            }
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerStageFightStartPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_stage_fight_start_payload::ServerStageFightStartBody,
    >,
}
/// Nested message and enum types in `ServerStageFightStartPayload`.
pub mod server_stage_fight_start_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerStageFightStartBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_stage_fight_start_body::ServerStageFightStartATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerStageFightStartBody`.
    pub mod server_stage_fight_start_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerStageFightStartATar {
            #[prost(int64, tag = "1")]
            pub stage_id: i64,
            #[prost(string, tag = "2")]
            pub stage_name: ::prost::alloc::string::String,
            #[prost(int64, tag = "3")]
            pub battle_id: i64,
            #[prost(string, tag = "4")]
            pub battle_unique_id: ::prost::alloc::string::String,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerStageFightEndPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_stage_fight_end_payload::ServerStageFightEndBody,
    >,
}
/// Nested message and enum types in `ServerStageFightEndPayload`.
pub mod server_stage_fight_end_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerStageFightEndBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_stage_fight_end_body::ServerStageFightEndATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerStageFightEndBody`.
    pub mod server_stage_fight_end_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerStageFightEndATar {
            #[prost(int64, tag = "1")]
            pub stage_id: i64,
            #[prost(string, tag = "2")]
            pub stage_name: ::prost::alloc::string::String,
            #[prost(int64, tag = "3")]
            pub battle_id: i64,
            #[prost(message, optional, tag = "4")]
            pub team: ::core::option::Option<super::super::Team>,
            #[prost(enumeration = "super::PveResult", tag = "5")]
            pub result: i32,
            #[prost(int64, tag = "6")]
            pub time: i64,
            #[prost(message, repeated, tag = "7")]
            pub card_info: ::prost::alloc::vec::Vec<super::super::CardInfo>,
            #[prost(message, optional, tag = "8")]
            pub fight_info: ::core::option::Option<super::super::FightInfo>,
            #[prost(message, optional, tag = "9")]
            pub result_info: ::core::option::Option<super::super::FightResultInfo>,
            #[prost(message, repeated, tag = "10")]
            pub equip_info: ::prost::alloc::vec::Vec<super::super::PlayerEquipInfo>,
            #[prost(message, repeated, tag = "11")]
            pub phoneme_info: ::prost::alloc::vec::Vec<super::super::PlayerPhonemeInfo>,
            #[prost(string, tag = "12")]
            pub battle_unique_id: ::prost::alloc::string::String,
        }
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum PveResult {
        InvalidPveResult = 0,
        Success = 1,
        Fail = 2,
    }
    impl PveResult {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PveResult::InvalidPveResult => "InvalidPveResult",
                PveResult::Success => "PveResultSuccess",
                PveResult::Fail => "PveResultFail",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "InvalidPveResult" => Some(Self::InvalidPveResult),
                "PveResultSuccess" => Some(Self::Success),
                "PveResultFail" => Some(Self::Fail),
                _ => None,
            }
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerStagePlotPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<server_stage_plot_payload::ServerStagePlotBody>,
}
/// Nested message and enum types in `ServerStagePlotPayload`.
pub mod server_stage_plot_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerStagePlotBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<server_stage_plot_body::ServerStagePlotATar>,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerStagePlotBody`.
    pub mod server_stage_plot_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerStagePlotATar {
            #[prost(int64, tag = "1")]
            pub stage_id: i64,
            #[prost(string, tag = "2")]
            pub stage_name: ::prost::alloc::string::String,
            #[prost(int64, tag = "3")]
            pub battle_id: i64,
            #[prost(int64, tag = "4")]
            pub time: i64,
            #[prost(message, optional, tag = "5")]
            pub plot_info: ::core::option::Option<super::super::PlotInfo>,
            #[prost(string, tag = "6")]
            pub flag: ::prost::alloc::string::String,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerStageFinishPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<server_stage_finish_payload::ServerStageFinishBody>,
}
/// Nested message and enum types in `ServerStageFinishPayload`.
pub mod server_stage_finish_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerStageFinishBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_stage_finish_body::ServerStageFinishATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerStageFinishBody`.
    pub mod server_stage_finish_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerStageFinishATar {
            #[prost(int64, tag = "1")]
            pub stage_id: i64,
            #[prost(string, tag = "2")]
            pub stage_name: ::prost::alloc::string::String,
            #[prost(int64, tag = "3")]
            pub battle_id: i64,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerExploreUnlockPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_explore_unlock_payload::ServerExploreUnlockBody,
    >,
}
/// Nested message and enum types in `ServerExploreUnlockPayload`.
pub mod server_explore_unlock_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerExploreUnlockBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_explore_unlock_body::ServerExploreUnlockATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerExploreUnlockBody`.
    pub mod server_explore_unlock_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerExploreUnlockATar {
            #[prost(int64, tag = "1")]
            pub explore_id: i64,
            #[prost(string, tag = "2")]
            pub explore_name: ::prost::alloc::string::String,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerExploreStartPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_explore_start_payload::ServerExploreStartBody,
    >,
}
/// Nested message and enum types in `ServerExploreStartPayload`.
pub mod server_explore_start_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerExploreStartBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_explore_start_body::ServerExploreStartATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerExploreStartBody`.
    pub mod server_explore_start_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerExploreStartATar {
            #[prost(int64, tag = "1")]
            pub explore_id: i64,
            #[prost(string, tag = "2")]
            pub explore_name: ::prost::alloc::string::String,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerExploreTaskPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<server_explore_task_payload::ServerExploreTaskBody>,
}
/// Nested message and enum types in `ServerExploreTaskPayload`.
pub mod server_explore_task_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerExploreTaskBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_explore_task_body::ServerExploreTaskATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerExploreTaskBody`.
    pub mod server_explore_task_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerExploreTaskATar {
            #[prost(int64, tag = "1")]
            pub explore_id: i64,
            #[prost(string, tag = "2")]
            pub explore_name: ::prost::alloc::string::String,
            #[prost(int64, tag = "3")]
            pub task_id: i64,
            #[prost(string, tag = "4")]
            pub task_name: ::prost::alloc::string::String,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerUseKeywordPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<server_use_keyword_payload::ServerUseKeywordBody>,
}
/// Nested message and enum types in `ServerUseKeywordPayload`.
pub mod server_use_keyword_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerUseKeywordBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<server_use_keyword_body::ServerUseKeywordATar>,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerUseKeywordBody`.
    pub mod server_use_keyword_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerUseKeywordATar {
            #[prost(int64, tag = "1")]
            pub card_id: i64,
            #[prost(string, tag = "2")]
            pub card_name: ::prost::alloc::string::String,
            #[prost(int64, tag = "3")]
            pub keyword: i64,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerExploreFinishPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_explore_finish_payload::ServerExploreFinishBody,
    >,
}
/// Nested message and enum types in `ServerExploreFinishPayload`.
pub mod server_explore_finish_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerExploreFinishBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_explore_finish_body::ServerExploreFinishATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerExploreFinishBody`.
    pub mod server_explore_finish_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerExploreFinishATar {
            #[prost(int64, tag = "1")]
            pub explore_id: i64,
            #[prost(string, tag = "2")]
            pub explore_name: ::prost::alloc::string::String,
            #[prost(message, optional, tag = "3")]
            pub fight_info: ::core::option::Option<super::super::FightInfo>,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerExploreFreePayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<server_explore_free_payload::ServerExploreFreeBody>,
}
/// Nested message and enum types in `ServerExploreFreePayload`.
pub mod server_explore_free_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerExploreFreeBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_explore_free_body::ServerExploreFreeATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerExploreFreeBody`.
    pub mod server_explore_free_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerExploreFreeATar {
            #[prost(int64, tag = "1")]
            pub explore_id: i64,
            #[prost(string, tag = "2")]
            pub explore_name: ::prost::alloc::string::String,
            #[prost(message, optional, tag = "3")]
            pub fight_info: ::core::option::Option<super::super::FightInfo>,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerExplorePuzzlePayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_explore_puzzle_payload::ServerExplorePuzzleBody,
    >,
}
/// Nested message and enum types in `ServerExplorePuzzlePayload`.
pub mod server_explore_puzzle_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerExplorePuzzleBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_explore_puzzle_body::ServerExplorePuzzleATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerExplorePuzzleBody`.
    pub mod server_explore_puzzle_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerExplorePuzzleATar {
            #[prost(int64, tag = "1")]
            pub puzzle_id: i64,
            #[prost(string, tag = "2")]
            pub puzzle_name: ::prost::alloc::string::String,
            #[prost(string, tag = "3")]
            pub is_first: ::prost::alloc::string::String,
            #[prost(enumeration = "super::PuzzleResult", tag = "4")]
            pub result: i32,
        }
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum PuzzleResult {
        InvalidResult = 0,
        Win = 1,
        Lose = 2,
    }
    impl PuzzleResult {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PuzzleResult::InvalidResult => "Invalid_Result",
                PuzzleResult::Win => "Win",
                PuzzleResult::Lose => "Lose",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "Invalid_Result" => Some(Self::InvalidResult),
                "Win" => Some(Self::Win),
                "Lose" => Some(Self::Lose),
                _ => None,
            }
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerExplorePuzzleDrinkMakingPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_explore_puzzle_drink_making_payload::ServerExplorePuzzleDrinkMakingBody,
    >,
}
/// Nested message and enum types in `ServerExplorePuzzleDrinkMakingPayload`.
pub mod server_explore_puzzle_drink_making_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerExplorePuzzleDrinkMakingBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_explore_puzzle_drink_making_body::ServerExplorePuzzleDrinkMakingATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerExplorePuzzleDrinkMakingBody`.
    pub mod server_explore_puzzle_drink_making_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerExplorePuzzleDrinkMakingATar {
            #[prost(int64, tag = "1")]
            pub puzzle_id: i64,
            #[prost(string, tag = "2")]
            pub puzzle_name: ::prost::alloc::string::String,
            #[prost(int64, tag = "3")]
            pub drink_making_id: i64,
            #[prost(int64, repeated, tag = "4")]
            pub accessories_ids: ::prost::alloc::vec::Vec<i64>,
            #[prost(int64, tag = "5")]
            pub mark: i64,
            #[prost(enumeration = "super::DrinkMakingLevel", tag = "6")]
            pub level: i32,
        }
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum DrinkMakingLevel {
        InvalidLevel = 0,
        LevelB = 1,
        LevelA = 2,
        LevelS = 3,
    }
    impl DrinkMakingLevel {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DrinkMakingLevel::InvalidLevel => "Invalid_level",
                DrinkMakingLevel::LevelB => "Level_B",
                DrinkMakingLevel::LevelA => "Level_A",
                DrinkMakingLevel::LevelS => "Level_S",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "Invalid_level" => Some(Self::InvalidLevel),
                "Level_B" => Some(Self::LevelB),
                "Level_A" => Some(Self::LevelA),
                "Level_S" => Some(Self::LevelS),
                _ => None,
            }
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerPresentSendPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<server_present_send_payload::ServerPresentSendBody>,
}
/// Nested message and enum types in `ServerPresentSendPayload`.
pub mod server_present_send_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerPresentSendBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_present_send_body::ServerPresentSendATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerPresentSendBody`.
    pub mod server_present_send_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerPresentSendATar {
            #[prost(int64, tag = "1")]
            pub card_id: i64,
            #[prost(string, tag = "2")]
            pub card_name: ::prost::alloc::string::String,
            #[prost(int64, tag = "3")]
            pub present_id: i64,
            #[prost(string, tag = "4")]
            pub present_name: ::prost::alloc::string::String,
            #[prost(int64, tag = "5")]
            pub present_num: i64,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerExploreGachaMachinePayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_explore_gacha_machine_payload::ServerExploreGachaMachineBody,
    >,
}
/// Nested message and enum types in `ServerExploreGachaMachinePayload`.
pub mod server_explore_gacha_machine_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerExploreGachaMachineBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_explore_gacha_machine_body::ServerExploreGachaMachineATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerExploreGachaMachineBody`.
    pub mod server_explore_gacha_machine_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerExploreGachaMachineATar {}
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerAutoDrinkMakingPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_auto_drink_making_payload::ServerAutoDrinkMakingBody,
    >,
}
/// Nested message and enum types in `ServerAutoDrinkMakingPayload`.
pub mod server_auto_drink_making_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerAutoDrinkMakingBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_auto_drink_making_body::ServerAutoDrinkMakingATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerAutoDrinkMakingBody`.
    pub mod server_auto_drink_making_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerAutoDrinkMakingATar {
            #[prost(int64, tag = "1")]
            pub puzzle_id: i64,
            #[prost(string, tag = "2")]
            pub puzzle_name: ::prost::alloc::string::String,
            #[prost(int64, tag = "3")]
            pub drink_making_id: i64,
            #[prost(int64, repeated, tag = "4")]
            pub accessories_ids: ::prost::alloc::vec::Vec<i64>,
            #[prost(int64, tag = "5")]
            pub times: i64,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerHometownShowPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_hometown_show_payload::ServerHometownShowBody,
    >,
}
/// Nested message and enum types in `ServerHometownShowPayload`.
pub mod server_hometown_show_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerHometownShowBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_hometown_show_body::ServerHometownShowATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerHometownShowBody`.
    pub mod server_hometown_show_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerHometownShowATar {
            #[prost(int64, tag = "1")]
            pub character_id: i64,
            #[prost(int64, tag = "2")]
            pub npc_id: i64,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerHometownDailyPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_hometown_daily_payload::ServerHometownDailyBody,
    >,
}
/// Nested message and enum types in `ServerHometownDailyPayload`.
pub mod server_hometown_daily_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerHometownDailyBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_hometown_daily_body::ServerHometownDailyATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerHometownDailyBody`.
    pub mod server_hometown_daily_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerHometownDailyATar {
            #[prost(int64, repeated, tag = "1")]
            pub daily: ::prost::alloc::vec::Vec<i64>,
            #[prost(int64, tag = "2")]
            pub finish: i64,
            #[prost(int64, tag = "3")]
            pub count: i64,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerBattleStarsPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<server_battle_stars_payload::ServerBattleStarsBody>,
}
/// Nested message and enum types in `ServerBattleStarsPayload`.
pub mod server_battle_stars_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerBattleStarsBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_battle_stars_body::ServerBattleStarsATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerBattleStarsBody`.
    pub mod server_battle_stars_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerBattleStarsATar {
            #[prost(int64, repeated, tag = "1")]
            pub pre_star_num: ::prost::alloc::vec::Vec<i64>,
            #[prost(int64, repeated, tag = "2")]
            pub new_star_num: ::prost::alloc::vec::Vec<i64>,
            #[prost(int64, tag = "3")]
            pub battle_id: i64,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerBattleCheatPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<server_battle_cheat_payload::ServerBattleCheatBody>,
}
/// Nested message and enum types in `ServerBattleCheatPayload`.
pub mod server_battle_cheat_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerBattleCheatBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_battle_cheat_body::ServerBattleCheatATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerBattleCheatBody`.
    pub mod server_battle_cheat_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerBattleCheatATar {
            #[prost(int64, tag = "1")]
            pub battle_id: i64,
            #[prost(string, tag = "2")]
            pub reason: ::prost::alloc::string::String,
            #[prost(string, tag = "3")]
            pub battle_unique_id: ::prost::alloc::string::String,
            #[prost(int64, tag = "4")]
            pub r#type: i64,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerMusicFightPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<server_music_fight_payload::ServerMusicFightBody>,
}
/// Nested message and enum types in `ServerMusicFightPayload`.
pub mod server_music_fight_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerMusicFightBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<server_music_fight_body::ServerMusicFightATar>,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerMusicFightBody`.
    pub mod server_music_fight_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerMusicFightATar {
            #[prost(int64, tag = "1")]
            pub music_id: i64,
            #[prost(string, tag = "2")]
            pub music_name: ::prost::alloc::string::String,
            #[prost(message, optional, tag = "3")]
            pub team: ::core::option::Option<super::super::Team>,
            #[prost(enumeration = "super::MusicResult", tag = "4")]
            pub result: i32,
            #[prost(message, repeated, tag = "5")]
            pub card_info: ::prost::alloc::vec::Vec<super::super::CardInfo>,
            #[prost(message, optional, tag = "6")]
            pub fight_info: ::core::option::Option<super::super::FightInfo>,
            #[prost(message, optional, tag = "7")]
            pub result_info: ::core::option::Option<super::super::FightResultInfo>,
        }
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum MusicResult {
        InvalidMusicResult = 0,
        MusicResult1Star = 1,
        MusicResult2Star = 2,
        MusicResult3Star = 3,
        Fail = 4,
    }
    impl MusicResult {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                MusicResult::InvalidMusicResult => "InvalidMusicResult",
                MusicResult::MusicResult1Star => "MusicResult1Star",
                MusicResult::MusicResult2Star => "MusicResult2Star",
                MusicResult::MusicResult3Star => "MusicResult3Star",
                MusicResult::Fail => "MusicResultFail",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "InvalidMusicResult" => Some(Self::InvalidMusicResult),
                "MusicResult1Star" => Some(Self::MusicResult1Star),
                "MusicResult2Star" => Some(Self::MusicResult2Star),
                "MusicResult3Star" => Some(Self::MusicResult3Star),
                "MusicResultFail" => Some(Self::Fail),
                _ => None,
            }
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerMusicSweepPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<server_music_sweep_payload::ServerMusicSweepBody>,
}
/// Nested message and enum types in `ServerMusicSweepPayload`.
pub mod server_music_sweep_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerMusicSweepBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<server_music_sweep_body::ServerMusicSweepATar>,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerMusicSweepBody`.
    pub mod server_music_sweep_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerMusicSweepATar {
            #[prost(int64, tag = "1")]
            pub music_id: i64,
            #[prost(string, tag = "2")]
            pub music_name: ::prost::alloc::string::String,
            #[prost(enumeration = "super::super::SweepType", tag = "3")]
            pub r#type: i32,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerDqFightPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<server_dq_fight_payload::ServerDqFightBody>,
}
/// Nested message and enum types in `ServerDqFightPayload`.
pub mod server_dq_fight_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerDqFightBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<server_dq_fight_body::ServerDqFightATar>,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerDqFightBody`.
    pub mod server_dq_fight_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerDqFightATar {
            #[prost(int64, tag = "1")]
            pub dq_id: i64,
            #[prost(string, tag = "2")]
            pub dq_name: ::prost::alloc::string::String,
            #[prost(message, optional, tag = "3")]
            pub team: ::core::option::Option<super::super::Team>,
            #[prost(enumeration = "super::DqResult", tag = "4")]
            pub result: i32,
            #[prost(message, repeated, tag = "5")]
            pub card_info: ::prost::alloc::vec::Vec<super::super::CardInfo>,
            #[prost(message, optional, tag = "6")]
            pub fight_info: ::core::option::Option<super::super::FightInfo>,
            #[prost(message, optional, tag = "7")]
            pub result_info: ::core::option::Option<super::super::FightResultInfo>,
            #[prost(int64, tag = "8")]
            pub time: i64,
            #[prost(int64, tag = "9")]
            pub stamina_cost: i64,
            #[prost(string, tag = "10")]
            pub is_first: ::prost::alloc::string::String,
            #[prost(int64, repeated, tag = "11")]
            pub star: ::prost::alloc::vec::Vec<i64>,
            #[prost(string, tag = "12")]
            pub is_first_three_star: ::prost::alloc::string::String,
        }
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum DqResult {
        InvalidDqResult = 0,
        Success = 1,
        Fail = 2,
    }
    impl DqResult {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DqResult::InvalidDqResult => "InvalidDqResult",
                DqResult::Success => "DqResultSuccess",
                DqResult::Fail => "DqResultFail",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "InvalidDqResult" => Some(Self::InvalidDqResult),
                "DqResultSuccess" => Some(Self::Success),
                "DqResultFail" => Some(Self::Fail),
                _ => None,
            }
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerDqFightStartPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_dq_fight_start_payload::ServerDqFightStartBody,
    >,
}
/// Nested message and enum types in `ServerDqFightStartPayload`.
pub mod server_dq_fight_start_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerDqFightStartBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_dq_fight_start_body::ServerDqFightStartATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerDqFightStartBody`.
    pub mod server_dq_fight_start_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerDqFightStartATar {
            #[prost(int64, tag = "1")]
            pub dq_id: i64,
            #[prost(int64, tag = "2")]
            pub battle_id: i64,
            #[prost(string, tag = "3")]
            pub battle_unique_id: ::prost::alloc::string::String,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerDqFightEndPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<server_dq_fight_end_payload::ServerDqFightEndBody>,
}
/// Nested message and enum types in `ServerDqFightEndPayload`.
pub mod server_dq_fight_end_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerDqFightEndBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_dq_fight_end_body::ServerDqFightEndATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerDqFightEndBody`.
    pub mod server_dq_fight_end_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerDqFightEndATar {
            #[prost(int64, tag = "1")]
            pub dq_id: i64,
            #[prost(string, tag = "2")]
            pub dq_name: ::prost::alloc::string::String,
            #[prost(message, optional, tag = "3")]
            pub team: ::core::option::Option<super::super::Team>,
            #[prost(enumeration = "super::DqResult", tag = "4")]
            pub result: i32,
            #[prost(message, repeated, tag = "5")]
            pub card_info: ::prost::alloc::vec::Vec<super::super::CardInfo>,
            #[prost(message, optional, tag = "6")]
            pub fight_info: ::core::option::Option<super::super::FightInfo>,
            #[prost(message, optional, tag = "7")]
            pub result_info: ::core::option::Option<super::super::FightResultInfo>,
            #[prost(int64, tag = "8")]
            pub time: i64,
            #[prost(int64, tag = "9")]
            pub stamina_cost: i64,
            #[prost(string, tag = "10")]
            pub is_first: ::prost::alloc::string::String,
            #[prost(int64, repeated, tag = "11")]
            pub star: ::prost::alloc::vec::Vec<i64>,
            #[prost(string, tag = "12")]
            pub is_first_three_star: ::prost::alloc::string::String,
            #[prost(string, tag = "13")]
            pub battle_unique_id: ::prost::alloc::string::String,
            #[prost(message, repeated, tag = "14")]
            pub equip_info: ::prost::alloc::vec::Vec<super::super::PlayerEquipInfo>,
            #[prost(message, repeated, tag = "15")]
            pub phoneme_info: ::prost::alloc::vec::Vec<super::super::PlayerPhonemeInfo>,
            #[prost(int64, tag = "16")]
            pub battle_id: i64,
            #[prost(int64, tag = "17")]
            pub bonus_times: i64,
        }
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum DqResult {
        InvalidDqResult = 0,
        Success = 1,
        Fail = 2,
    }
    impl DqResult {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DqResult::InvalidDqResult => "InvalidDqResult",
                DqResult::Success => "DqResultSuccess",
                DqResult::Fail => "DqResultFail",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "InvalidDqResult" => Some(Self::InvalidDqResult),
                "DqResultSuccess" => Some(Self::Success),
                "DqResultFail" => Some(Self::Fail),
                _ => None,
            }
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerDqSweepPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<server_dq_sweep_payload::ServerDqSweepBody>,
}
/// Nested message and enum types in `ServerDqSweepPayload`.
pub mod server_dq_sweep_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerDqSweepBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<server_dq_sweep_body::ServerDqSweepATar>,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerDqSweepBody`.
    pub mod server_dq_sweep_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerDqSweepATar {
            #[prost(int64, tag = "1")]
            pub dq_id: i64,
            #[prost(string, tag = "2")]
            pub dq_name: ::prost::alloc::string::String,
            #[prost(enumeration = "super::super::SweepType", tag = "3")]
            pub r#type: i32,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerDqSweepStartPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_dq_sweep_start_payload::ServerDqSweepStartBody,
    >,
}
/// Nested message and enum types in `ServerDqSweepStartPayload`.
pub mod server_dq_sweep_start_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerDqSweepStartBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_dq_sweep_start_body::ServerDqSweepStartATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerDqSweepStartBody`.
    pub mod server_dq_sweep_start_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerDqSweepStartATar {
            #[prost(int64, tag = "1")]
            pub dq_id: i64,
            #[prost(string, tag = "2")]
            pub dq_name: ::prost::alloc::string::String,
            #[prost(int64, tag = "3")]
            pub times: i64,
            #[prost(int64, tag = "4")]
            pub stamina_cost: i64,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerDqSweepFinishPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_dq_sweep_finish_payload::ServerDqSweepFinishBody,
    >,
}
/// Nested message and enum types in `ServerDqSweepFinishPayload`.
pub mod server_dq_sweep_finish_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerDqSweepFinishBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_dq_sweep_finish_body::ServerDqSweepFinishATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerDqSweepFinishBody`.
    pub mod server_dq_sweep_finish_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerDqSweepFinishATar {
            #[prost(int64, tag = "1")]
            pub dq_id: i64,
            #[prost(string, tag = "2")]
            pub dq_name: ::prost::alloc::string::String,
            #[prost(int64, tag = "3")]
            pub expire_times: i64,
            #[prost(int64, tag = "4")]
            pub times: i64,
            #[prost(enumeration = "FinishStatus", tag = "5")]
            pub status: i32,
            #[prost(int64, tag = "6")]
            pub duration: i64,
            #[prost(int64, tag = "7")]
            pub return_stamina: i64,
            #[prost(int64, tag = "8")]
            pub bonus_times: i64,
        }
        #[derive(serde::Serialize, serde::Deserialize)]
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::prost::Enumeration
        )]
        #[repr(i32)]
        pub enum FinishStatus {
            Invalid = 0,
            Finish = 1,
            Interruption = 2,
        }
        impl FinishStatus {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    FinishStatus::Invalid => "Invalid",
                    FinishStatus::Finish => "Finish",
                    FinishStatus::Interruption => "Interruption",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "Invalid" => Some(Self::Invalid),
                    "Finish" => Some(Self::Finish),
                    "Interruption" => Some(Self::Interruption),
                    _ => None,
                }
            }
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerCardSnapshotPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_card_snapshot_payload::ServerCardSnapshotBody,
    >,
}
/// Nested message and enum types in `ServerCardSnapshotPayload`.
pub mod server_card_snapshot_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerCardSnapshotBody {
        #[prost(message, repeated, tag = "1")]
        pub a_tar: ::prost::alloc::vec::Vec<
            server_card_snapshot_body::ServerCardSnapshotATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerCardSnapshotBody`.
    pub mod server_card_snapshot_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerCardSnapshotATar {
            #[prost(int64, tag = "1")]
            pub card_id: i64,
            #[prost(string, tag = "2")]
            pub card_name: ::prost::alloc::string::String,
            #[prost(message, optional, tag = "3")]
            pub card_info: ::core::option::Option<super::super::CardInfo>,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerCardLevelUpPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_card_level_up_payload::ServerCardLevelUpBody,
    >,
}
/// Nested message and enum types in `ServerCardLevelUpPayload`.
pub mod server_card_level_up_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerCardLevelUpBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_card_level_up_body::ServerCardLevelUpATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerCardLevelUpBody`.
    pub mod server_card_level_up_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerCardLevelUpATar {
            #[prost(int64, tag = "1")]
            pub card_id: i64,
            #[prost(string, tag = "2")]
            pub card_name: ::prost::alloc::string::String,
            #[prost(int64, tag = "3")]
            pub level_before: i64,
            #[prost(int64, tag = "4")]
            pub level_after: i64,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerCardStarUpPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<server_card_star_up_payload::ServerCardStarUpBody>,
}
/// Nested message and enum types in `ServerCardStarUpPayload`.
pub mod server_card_star_up_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerCardStarUpBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_card_star_up_body::ServerCardStarUpATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerCardStarUpBody`.
    pub mod server_card_star_up_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerCardStarUpATar {
            #[prost(int64, tag = "1")]
            pub card_id: i64,
            #[prost(string, tag = "2")]
            pub card_name: ::prost::alloc::string::String,
            #[prost(int64, tag = "3")]
            pub star_before: i64,
            #[prost(int64, tag = "4")]
            pub star_after: i64,
            #[prost(int64, repeated, tag = "5")]
            pub before_sub_rank: ::prost::alloc::vec::Vec<i64>,
            #[prost(int64, repeated, tag = "6")]
            pub after_sub_rank: ::prost::alloc::vec::Vec<i64>,
            #[prost(int64, tag = "7")]
            pub active_sub_rank: i64,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerCardResetRankPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_card_reset_rank_payload::ServerCardResetRankBody,
    >,
}
/// Nested message and enum types in `ServerCardResetRankPayload`.
pub mod server_card_reset_rank_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerCardResetRankBody {
        #[prost(message, repeated, tag = "1")]
        pub a_tar: ::prost::alloc::vec::Vec<
            server_card_reset_rank_body::ServerCardResetRankATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerCardResetRankBody`.
    pub mod server_card_reset_rank_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerCardResetRankATar {
            #[prost(int64, tag = "1")]
            pub card_id: i64,
            #[prost(string, tag = "2")]
            pub card_name: ::prost::alloc::string::String,
            #[prost(int64, tag = "3")]
            pub rank_before: i64,
            #[prost(int64, tag = "4")]
            pub rank_after: i64,
            #[prost(int64, repeated, tag = "5")]
            pub before_sub_rank: ::prost::alloc::vec::Vec<i64>,
            #[prost(int64, repeated, tag = "6")]
            pub after_sub_rank: ::prost::alloc::vec::Vec<i64>,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerCardGradeUpPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_card_grade_up_payload::ServerCardGradeUpBody,
    >,
}
/// Nested message and enum types in `ServerCardGradeUpPayload`.
pub mod server_card_grade_up_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerCardGradeUpBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_card_grade_up_body::ServerCardGradeUpATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerCardGradeUpBody`.
    pub mod server_card_grade_up_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerCardGradeUpATar {
            #[prost(int64, tag = "1")]
            pub card_id: i64,
            #[prost(string, tag = "2")]
            pub card_name: ::prost::alloc::string::String,
            #[prost(int64, tag = "3")]
            pub grade_before: i64,
            #[prost(int64, tag = "4")]
            pub grade_after: i64,
            #[prost(int64, tag = "5")]
            pub panel_id: i64,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerCardFeelingPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<server_card_feeling_payload::ServerCardFeelingBody>,
}
/// Nested message and enum types in `ServerCardFeelingPayload`.
pub mod server_card_feeling_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerCardFeelingBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_card_feeling_body::ServerCardFeelingATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerCardFeelingBody`.
    pub mod server_card_feeling_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerCardFeelingATar {
            #[prost(int64, tag = "1")]
            pub card_id: i64,
            #[prost(string, tag = "2")]
            pub card_name: ::prost::alloc::string::String,
            #[prost(int64, tag = "3")]
            pub feeling_before: i64,
            #[prost(int64, tag = "4")]
            pub feeling_after: i64,
            #[prost(enumeration = "super::Reason", tag = "5")]
            pub reason: i32,
            #[prost(enumeration = "super::FeelingType", tag = "6")]
            pub r#type: i32,
            #[prost(int64, tag = "7")]
            pub bond_level_before: i64,
            #[prost(int64, tag = "8")]
            pub bond_level_after: i64,
        }
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Reason {
        InvalidReason = 0,
        BondsUpReasonGiftSending = 1,
        BondsUpReasonChapterPlot = 2,
        BondsUpReasonUseKeyword = 3,
    }
    impl Reason {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Reason::InvalidReason => "InvalidReason",
                Reason::BondsUpReasonGiftSending => "BondsUpReasonGiftSending",
                Reason::BondsUpReasonChapterPlot => "BondsUpReasonChapterPlot",
                Reason::BondsUpReasonUseKeyword => "BondsUpReasonUseKeyword",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "InvalidReason" => Some(Self::InvalidReason),
                "BondsUpReasonGiftSending" => Some(Self::BondsUpReasonGiftSending),
                "BondsUpReasonChapterPlot" => Some(Self::BondsUpReasonChapterPlot),
                "BondsUpReasonUseKeyword" => Some(Self::BondsUpReasonUseKeyword),
                _ => None,
            }
        }
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum FeelingType {
        InValidFeelingType = 0,
        Up = 1,
        Unchanged = 2,
    }
    impl FeelingType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                FeelingType::InValidFeelingType => "InValidFeelingType",
                FeelingType::Up => "FeelingTypeUp",
                FeelingType::Unchanged => "FeelingTypeUnchanged",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "InValidFeelingType" => Some(Self::InValidFeelingType),
                "FeelingTypeUp" => Some(Self::Up),
                "FeelingTypeUnchanged" => Some(Self::Unchanged),
                _ => None,
            }
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerCardBondRewardPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_card_bond_reward_payload::ServerCardBondRewardBody,
    >,
}
/// Nested message and enum types in `ServerCardBondRewardPayload`.
pub mod server_card_bond_reward_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerCardBondRewardBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_card_bond_reward_body::ServerCardBondRewardATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerCardBondRewardBody`.
    pub mod server_card_bond_reward_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerCardBondRewardATar {
            #[prost(int64, tag = "1")]
            pub card_id: i64,
            #[prost(string, tag = "2")]
            pub card_name: ::prost::alloc::string::String,
            #[prost(int64, tag = "3")]
            pub reward_level: i64,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerContractUnlockPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_contract_unlock_payload::ServerContractUnlockBody,
    >,
}
/// Nested message and enum types in `ServerContractUnlockPayload`.
pub mod server_contract_unlock_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerContractUnlockBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_contract_unlock_body::ServerContractUnlockATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerContractUnlockBody`.
    pub mod server_contract_unlock_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerContractUnlockATar {
            #[prost(int64, tag = "1")]
            pub card_id: i64,
            #[prost(string, tag = "2")]
            pub card_name: ::prost::alloc::string::String,
            #[prost(int64, tag = "3")]
            pub bond_level: i64,
            #[prost(int64, tag = "4")]
            pub skill_id: i64,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerEquipSnapshotPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_equip_snapshot_payload::ServerEquipSnapshotBody,
    >,
}
/// Nested message and enum types in `ServerEquipSnapshotPayload`.
pub mod server_equip_snapshot_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerEquipSnapshotBody {
        #[prost(message, repeated, tag = "1")]
        pub a_tar: ::prost::alloc::vec::Vec<
            server_equip_snapshot_body::ServerEquipSnapshotATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerEquipSnapshotBody`.
    pub mod server_equip_snapshot_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerEquipSnapshotATar {
            #[prost(int64, tag = "1")]
            pub equip_id: i64,
            #[prost(string, tag = "2")]
            pub unique_id: ::prost::alloc::string::String,
            #[prost(string, tag = "3")]
            pub equip_name: ::prost::alloc::string::String,
            #[prost(message, optional, tag = "4")]
            pub equip_info: ::core::option::Option<super::super::EquipInfo>,
            #[prost(string, tag = "5")]
            pub is_delete: ::prost::alloc::string::String,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerEquipLevelUpPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_equip_level_up_payload::ServerEquipLevelUpBody,
    >,
}
/// Nested message and enum types in `ServerEquipLevelUpPayload`.
pub mod server_equip_level_up_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerEquipLevelUpBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_equip_level_up_body::ServerEquipLevelUpATar,
        >,
        #[prost(message, repeated, tag = "11")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerEquipLevelUpBody`.
    pub mod server_equip_level_up_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerEquipLevelUpATar {
            #[prost(int64, tag = "1")]
            pub equip_id: i64,
            #[prost(string, tag = "2")]
            pub unique_id: ::prost::alloc::string::String,
            #[prost(string, tag = "3")]
            pub equip_name: ::prost::alloc::string::String,
            #[prost(int64, tag = "4")]
            pub level_before: i64,
            #[prost(int64, tag = "5")]
            pub level_after: i64,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerEquipStarUpPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_equip_star_up_payload::ServerEquipStarUpBody,
    >,
}
/// Nested message and enum types in `ServerEquipStarUpPayload`.
pub mod server_equip_star_up_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerEquipStarUpBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_equip_star_up_body::ServerEquipStarUpATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerEquipStarUpBody`.
    pub mod server_equip_star_up_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerEquipStarUpATar {
            #[prost(int64, tag = "1")]
            pub equip_id: i64,
            #[prost(string, tag = "2")]
            pub unique_id: ::prost::alloc::string::String,
            #[prost(string, tag = "3")]
            pub equip_name: ::prost::alloc::string::String,
            #[prost(int64, tag = "4")]
            pub star_before: i64,
            #[prost(int64, tag = "5")]
            pub star_after: i64,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerEquipGradeUpPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_equip_grade_up_payload::ServerEquipGradeUpBody,
    >,
}
/// Nested message and enum types in `ServerEquipGradeUpPayload`.
pub mod server_equip_grade_up_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerEquipGradeUpBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_equip_grade_up_body::ServerEquipGradeUpATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerEquipGradeUpBody`.
    pub mod server_equip_grade_up_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerEquipGradeUpATar {
            #[prost(int64, tag = "1")]
            pub equip_id: i64,
            #[prost(string, tag = "2")]
            pub unique_id: ::prost::alloc::string::String,
            #[prost(string, tag = "3")]
            pub equip_name: ::prost::alloc::string::String,
            #[prost(int64, tag = "4")]
            pub grade_before: i64,
            #[prost(int64, tag = "5")]
            pub grade_after: i64,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerEquipArrangePayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_equip_arrange_payload::ServerEquipArrangeBody,
    >,
}
/// Nested message and enum types in `ServerEquipArrangePayload`.
pub mod server_equip_arrange_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerEquipArrangeBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_equip_arrange_body::ServerEquipArrangeATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerEquipArrangeBody`.
    pub mod server_equip_arrange_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerEquipArrangeATar {
            #[prost(int64, tag = "1")]
            pub equip_id: i64,
            #[prost(string, tag = "2")]
            pub unique_id: ::prost::alloc::string::String,
            #[prost(string, tag = "3")]
            pub equip_name: ::prost::alloc::string::String,
            #[prost(int64, tag = "4")]
            pub card_id: i64,
            #[prost(string, tag = "5")]
            pub card_name: ::prost::alloc::string::String,
            #[prost(int64, tag = "6")]
            pub position: i64,
            #[prost(enumeration = "super::EquipArrangeType", tag = "7")]
            pub r#type: i32,
        }
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum EquipArrangeType {
        InvalidEquipArrangeType = 0,
        Fit = 1,
        Unfit = 2,
    }
    impl EquipArrangeType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                EquipArrangeType::InvalidEquipArrangeType => "InvalidEquipArrangeType",
                EquipArrangeType::Fit => "Fit",
                EquipArrangeType::Unfit => "Unfit",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "InvalidEquipArrangeType" => Some(Self::InvalidEquipArrangeType),
                "Fit" => Some(Self::Fit),
                "Unfit" => Some(Self::Unfit),
                _ => None,
            }
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerEquipWakeUpPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_equip_wake_up_payload::ServerEquipWakeUpBody,
    >,
}
/// Nested message and enum types in `ServerEquipWakeUpPayload`.
pub mod server_equip_wake_up_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerEquipWakeUpBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_equip_wake_up_body::ServerEquipWakeUpATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerEquipWakeUpBody`.
    pub mod server_equip_wake_up_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerEquipWakeUpATar {
            #[prost(int64, tag = "1")]
            pub new_equip_id: i64,
            #[prost(int64, repeated, tag = "2")]
            pub consume_equip_ids: ::prost::alloc::vec::Vec<i64>,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerEquipDecomposePayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_equip_decompose_payload::ServerEquipDecomposeBody,
    >,
}
/// Nested message and enum types in `ServerEquipDecomposePayload`.
pub mod server_equip_decompose_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerEquipDecomposeBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_equip_decompose_body::ServerEquipDecomposeATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerEquipDecomposeBody`.
    pub mod server_equip_decompose_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerEquipDecomposeATar {
            #[prost(int64, repeated, tag = "1")]
            pub decomposed_equipments: ::prost::alloc::vec::Vec<i64>,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerPhonemeSnapshotPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_phoneme_snapshot_payload::ServerPhonemeSnapshotBody,
    >,
}
/// Nested message and enum types in `ServerPhonemeSnapshotPayload`.
pub mod server_phoneme_snapshot_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerPhonemeSnapshotBody {
        #[prost(message, repeated, tag = "1")]
        pub a_tar: ::prost::alloc::vec::Vec<
            server_phoneme_snapshot_body::ServerPhonemeSnapshotATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerPhonemeSnapshotBody`.
    pub mod server_phoneme_snapshot_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerPhonemeSnapshotATar {
            #[prost(int64, tag = "1")]
            pub phoneme_id: i64,
            #[prost(string, tag = "2")]
            pub unique_id: ::prost::alloc::string::String,
            #[prost(string, tag = "3")]
            pub phoneme_name: ::prost::alloc::string::String,
            #[prost(message, optional, tag = "4")]
            pub phoneme_info: ::core::option::Option<super::super::PhonemeInfo>,
            #[prost(string, tag = "5")]
            pub is_delete: ::prost::alloc::string::String,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerPhonemeLevelUpPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_phoneme_level_up_payload::ServerPhonemeLevelUpBody,
    >,
}
/// Nested message and enum types in `ServerPhonemeLevelUpPayload`.
pub mod server_phoneme_level_up_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerPhonemeLevelUpBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_phoneme_level_up_body::ServerPhonemeLevelUpATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerPhonemeLevelUpBody`.
    pub mod server_phoneme_level_up_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerPhonemeLevelUpATar {
            #[prost(int64, tag = "1")]
            pub phoneme_id: i64,
            #[prost(string, tag = "2")]
            pub unique_id: ::prost::alloc::string::String,
            #[prost(string, tag = "3")]
            pub phoneme_name: ::prost::alloc::string::String,
            #[prost(int64, tag = "4")]
            pub level_before: i64,
            #[prost(int64, tag = "5")]
            pub level_after: i64,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerPhonemeFitPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<server_phoneme_fit_payload::ServerPhonemeFitBody>,
}
/// Nested message and enum types in `ServerPhonemeFitPayload`.
pub mod server_phoneme_fit_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerPhonemeFitBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<server_phoneme_fit_body::ServerPhonemeFitATar>,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerPhonemeFitBody`.
    pub mod server_phoneme_fit_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerPhonemeFitATar {
            #[prost(int64, tag = "1")]
            pub phoneme_id: i64,
            #[prost(string, tag = "2")]
            pub unique_id: ::prost::alloc::string::String,
            #[prost(string, tag = "3")]
            pub phone_name: ::prost::alloc::string::String,
            #[prost(int64, tag = "4")]
            pub card_id: i64,
            #[prost(string, tag = "5")]
            pub card_name: ::prost::alloc::string::String,
            #[prost(int64, tag = "6")]
            pub slot: i64,
            #[prost(enumeration = "super::PhonemeArrangeType", tag = "7")]
            pub r#type: i32,
        }
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum PhonemeArrangeType {
        InvalidPhonemeArrangeType = 0,
        Fit = 1,
        Unfit = 2,
    }
    impl PhonemeArrangeType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PhonemeArrangeType::InvalidPhonemeArrangeType => {
                    "InvalidPhonemeArrangeType"
                }
                PhonemeArrangeType::Fit => "Fit",
                PhonemeArrangeType::Unfit => "Unfit",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "InvalidPhonemeArrangeType" => Some(Self::InvalidPhonemeArrangeType),
                "Fit" => Some(Self::Fit),
                "Unfit" => Some(Self::Unfit),
                _ => None,
            }
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerPhonemeDecomposePayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_phoneme_decompose_payload::ServerPhonemeDecomposeBody,
    >,
}
/// Nested message and enum types in `ServerPhonemeDecomposePayload`.
pub mod server_phoneme_decompose_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerPhonemeDecomposeBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_phoneme_decompose_body::ServerPhonemeDecomposeATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerPhonemeDecomposeBody`.
    pub mod server_phoneme_decompose_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerPhonemeDecomposeATar {
            #[prost(int64, repeated, tag = "1")]
            pub decomposed_phonemes: ::prost::alloc::vec::Vec<i64>,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerPhonemeBuffGachaPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_phoneme_buff_gacha_payload::ServerPhonemeBuffGachaBody,
    >,
}
/// Nested message and enum types in `ServerPhonemeBuffGachaPayload`.
pub mod server_phoneme_buff_gacha_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerPhonemeBuffGachaBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_phoneme_buff_gacha_body::ServerPhonemeBuffGachaATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerPhonemeBuffGachaBody`.
    pub mod server_phoneme_buff_gacha_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerPhonemeBuffGachaATar {
            #[prost(int64, tag = "1")]
            pub phoneme_id: i64,
            #[prost(string, tag = "2")]
            pub unique_id: ::prost::alloc::string::String,
            #[prost(string, tag = "3")]
            pub phoneme_name: ::prost::alloc::string::String,
            #[prost(message, repeated, tag = "4")]
            pub buff_values: ::prost::alloc::vec::Vec<
                super::super::phoneme_info::BuffValue,
            >,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerPhonemeBuffConfirmPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_phoneme_buff_confirm_payload::ServerPhonemeBuffConfirmBody,
    >,
}
/// Nested message and enum types in `ServerPhonemeBuffConfirmPayload`.
pub mod server_phoneme_buff_confirm_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerPhonemeBuffConfirmBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_phoneme_buff_confirm_body::ServerPhonemeBuffConfirmATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerPhonemeBuffConfirmBody`.
    pub mod server_phoneme_buff_confirm_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerPhonemeBuffConfirmATar {
            #[prost(int64, tag = "1")]
            pub phoneme_id: i64,
            #[prost(string, tag = "2")]
            pub unique_id: ::prost::alloc::string::String,
            #[prost(string, tag = "3")]
            pub phoneme_name: ::prost::alloc::string::String,
            #[prost(message, repeated, tag = "4")]
            pub old_buff_values: ::prost::alloc::vec::Vec<
                super::super::phoneme_info::BuffValue,
            >,
            #[prost(message, repeated, tag = "5")]
            pub confirm_buff_values: ::prost::alloc::vec::Vec<
                super::super::phoneme_info::BuffValue,
            >,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerPhonemeSuitFitPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_phoneme_suit_fit_payload::ServerPhonemeSuitFitBody,
    >,
}
/// Nested message and enum types in `ServerPhonemeSuitFitPayload`.
pub mod server_phoneme_suit_fit_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerPhonemeSuitFitBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_phoneme_suit_fit_body::ServerPhonemeSuitFitATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerPhonemeSuitFitBody`.
    pub mod server_phoneme_suit_fit_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerPhonemeSuitFitATar {
            #[prost(int64, tag = "1")]
            pub suit_id: i64,
            #[prost(string, tag = "2")]
            pub suit_name: ::prost::alloc::string::String,
            #[prost(int64, tag = "3")]
            pub card_id: i64,
            #[prost(string, tag = "4")]
            pub card_name: ::prost::alloc::string::String,
            #[prost(enumeration = "super::SuitArrangeType", tag = "5")]
            pub r#type: i32,
        }
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum SuitArrangeType {
        InvalidSuitArrangeType = 0,
        Fit = 1,
        Unfit = 2,
    }
    impl SuitArrangeType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SuitArrangeType::InvalidSuitArrangeType => "InvalidSuitArrangeType",
                SuitArrangeType::Fit => "Fit",
                SuitArrangeType::Unfit => "Unfit",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "InvalidSuitArrangeType" => Some(Self::InvalidSuitArrangeType),
                "Fit" => Some(Self::Fit),
                "Unfit" => Some(Self::Unfit),
                _ => None,
            }
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerTaskReachPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<server_task_reach_payload::ServerTaskReachBody>,
}
/// Nested message and enum types in `ServerTaskReachPayload`.
pub mod server_task_reach_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerTaskReachBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<server_task_reach_body::ServerTaskReachATar>,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerTaskReachBody`.
    pub mod server_task_reach_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerTaskReachATar {
            #[prost(int64, tag = "1")]
            pub task_id: i64,
            #[prost(string, tag = "2")]
            pub task_name: ::prost::alloc::string::String,
            #[prost(enumeration = "super::super::TaskType", tag = "3")]
            pub r#type: i32,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerTaskRewardPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<server_task_reward_payload::ServerTaskRewardBody>,
}
/// Nested message and enum types in `ServerTaskRewardPayload`.
pub mod server_task_reward_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerTaskRewardBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<server_task_reward_body::ServerTaskRewardATar>,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerTaskRewardBody`.
    pub mod server_task_reward_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerTaskRewardATar {
            #[prost(int64, tag = "1")]
            pub task_id: i64,
            #[prost(string, tag = "2")]
            pub task_name: ::prost::alloc::string::String,
            #[prost(enumeration = "super::super::TaskType", tag = "3")]
            pub r#type: i32,
            #[prost(int64, tag = "4")]
            pub badge_id: i64,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerTaskAllRewardPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_task_all_reward_payload::ServerTaskAllRewardBody,
    >,
}
/// Nested message and enum types in `ServerTaskAllRewardPayload`.
pub mod server_task_all_reward_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerTaskAllRewardBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_task_all_reward_body::ServerTaskAllRewardATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerTaskAllRewardBody`.
    pub mod server_task_all_reward_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerTaskAllRewardATar {
            #[prost(int64, repeated, tag = "1")]
            pub task_ids: ::prost::alloc::vec::Vec<i64>,
            #[prost(enumeration = "super::super::TaskType", tag = "2")]
            pub r#type: i32,
            #[prost(int64, repeated, tag = "3")]
            pub badge_ids: ::prost::alloc::vec::Vec<i64>,
            #[prost(int64, tag = "4")]
            pub task_sub_type: i64,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerTaskPointRewardPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_task_point_reward_payload::ServerTaskPointRewardBody,
    >,
}
/// Nested message and enum types in `ServerTaskPointRewardPayload`.
pub mod server_task_point_reward_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerTaskPointRewardBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_task_point_reward_body::ServerTaskPointRewardATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerTaskPointRewardBody`.
    pub mod server_task_point_reward_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerTaskPointRewardATar {
            #[prost(int64, tag = "1")]
            pub task_point_reward_id: i64,
            #[prost(enumeration = "super::super::TaskType", tag = "2")]
            pub r#type: i32,
            #[prost(int64, tag = "3")]
            pub badge_id: i64,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerActivityCheckinRewardPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_activity_checkin_reward_payload::ServerActivityCheckinRewardBody,
    >,
}
/// Nested message and enum types in `ServerActivityCheckinRewardPayload`.
pub mod server_activity_checkin_reward_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerActivityCheckinRewardBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_activity_checkin_reward_body::ServerActivityCheckinRewardATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerActivityCheckinRewardBody`.
    pub mod server_activity_checkin_reward_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerActivityCheckinRewardATar {
            #[prost(int64, tag = "1")]
            pub checkin_id: i64,
            #[prost(int64, repeated, tag = "2")]
            pub reward_ids: ::prost::alloc::vec::Vec<i64>,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerActivityMissionRewardPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_activity_mission_reward_payload::ServerActivityMissionRewardBody,
    >,
}
/// Nested message and enum types in `ServerActivityMissionRewardPayload`.
pub mod server_activity_mission_reward_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerActivityMissionRewardBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_activity_mission_reward_body::ServerActivityMissionRewardATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerActivityMissionRewardBody`.
    pub mod server_activity_mission_reward_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerActivityMissionRewardATar {
            #[prost(int64, tag = "1")]
            pub mission_id: i64,
            #[prost(string, tag = "2")]
            pub mission_name: ::prost::alloc::string::String,
            #[prost(enumeration = "super::super::TaskType", tag = "3")]
            pub r#type: i32,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerActivityMissionAllRewardPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_activity_mission_all_reward_payload::ServerActivityMissionAllRewardBody,
    >,
}
/// Nested message and enum types in `ServerActivityMissionAllRewardPayload`.
pub mod server_activity_mission_all_reward_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerActivityMissionAllRewardBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_activity_mission_all_reward_body::ServerActivityMissionAllRewardATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerActivityMissionAllRewardBody`.
    pub mod server_activity_mission_all_reward_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerActivityMissionAllRewardATar {
            #[prost(int64, repeated, tag = "1")]
            pub mission_ids: ::prost::alloc::vec::Vec<i64>,
            #[prost(enumeration = "super::super::TaskType", tag = "2")]
            pub r#type: i32,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerNewbieReceiveLevelRewardPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_newbie_receive_level_reward_payload::ServerNewbieReceiveLevelRewardBody,
    >,
}
/// Nested message and enum types in `ServerNewbieReceiveLevelRewardPayload`.
pub mod server_newbie_receive_level_reward_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerNewbieReceiveLevelRewardBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_newbie_receive_level_reward_body::ServerNewbieReceiveLevelRewardATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerNewbieReceiveLevelRewardBody`.
    pub mod server_newbie_receive_level_reward_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerNewbieReceiveLevelRewardATar {
            #[prost(int64, tag = "1")]
            pub level_reward_master_id: i64,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerBadgeReceivePayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_badge_receive_payload::ServerBadgeReceiveBody,
    >,
}
/// Nested message and enum types in `ServerBadgeReceivePayload`.
pub mod server_badge_receive_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerBadgeReceiveBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_badge_receive_body::ServerBadgeReceiveATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerBadgeReceiveBody`.
    pub mod server_badge_receive_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerBadgeReceiveATar {
            #[prost(message, repeated, tag = "1")]
            pub badge_info: ::prost::alloc::vec::Vec<super::super::BadgeInfo>,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerEmailReceivePayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_email_receive_payload::ServerEmailReceiveBody,
    >,
}
/// Nested message and enum types in `ServerEmailReceivePayload`.
pub mod server_email_receive_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerEmailReceiveBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_email_receive_body::ServerEmailReceiveATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerEmailReceiveBody`.
    pub mod server_email_receive_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerEmailReceiveATar {
            #[prost(string, tag = "1")]
            pub email_id: ::prost::alloc::string::String,
            #[prost(string, tag = "2")]
            pub email_name: ::prost::alloc::string::String,
            #[prost(
                enumeration = "super::super::super::super::mail::v1::SenderType",
                tag = "3"
            )]
            pub sender_type: i32,
            #[prost(sint64, tag = "4")]
            pub remain: i64,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerEmailOpenPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<server_email_open_payload::ServerEmailOpenBody>,
}
/// Nested message and enum types in `ServerEmailOpenPayload`.
pub mod server_email_open_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerEmailOpenBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<server_email_open_body::ServerEmailOpenATar>,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerEmailOpenBody`.
    pub mod server_email_open_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerEmailOpenATar {
            #[prost(string, tag = "1")]
            pub email_id: ::prost::alloc::string::String,
            #[prost(string, tag = "2")]
            pub email_name: ::prost::alloc::string::String,
            #[prost(
                enumeration = "super::super::super::super::mail::v1::SenderType",
                tag = "3"
            )]
            pub sender_type: i32,
            #[prost(sint64, tag = "4")]
            pub remain: i64,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerEmailRewardPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<server_email_reward_payload::ServerEmailRewardBody>,
}
/// Nested message and enum types in `ServerEmailRewardPayload`.
pub mod server_email_reward_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerEmailRewardBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_email_reward_body::ServerEmailRewardATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerEmailRewardBody`.
    pub mod server_email_reward_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerEmailRewardATar {
            #[prost(string, tag = "1")]
            pub email_id: ::prost::alloc::string::String,
            #[prost(string, tag = "2")]
            pub email_name: ::prost::alloc::string::String,
            #[prost(
                enumeration = "super::super::super::super::mail::v1::SenderType",
                tag = "3"
            )]
            pub sender_type: i32,
            #[prost(sint64, tag = "4")]
            pub remain: i64,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerQuestionnaireSubmitPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_questionnaire_submit_payload::ServerQuestionnaireSubmitBody,
    >,
}
/// Nested message and enum types in `ServerQuestionnaireSubmitPayload`.
pub mod server_questionnaire_submit_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerQuestionnaireSubmitBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_questionnaire_submit_body::ServerQuestionnaireSubmitATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerQuestionnaireSubmitBody`.
    pub mod server_questionnaire_submit_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerQuestionnaireSubmitATar {
            #[prost(string, tag = "1")]
            pub questionnaire_id: ::prost::alloc::string::String,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerItemSnapshotPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_item_snapshot_payload::ServerItemSnapshotBody,
    >,
}
/// Nested message and enum types in `ServerItemSnapshotPayload`.
pub mod server_item_snapshot_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerItemSnapshotBody {
        #[prost(message, repeated, tag = "1")]
        pub a_tar: ::prost::alloc::vec::Vec<
            server_item_snapshot_body::ServerItemSnapshotATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerItemSnapshotBody`.
    pub mod server_item_snapshot_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerItemSnapshotATar {
            #[prost(int64, tag = "1")]
            pub item_id: i64,
            #[prost(string, tag = "2")]
            pub item_name: ::prost::alloc::string::String,
            #[prost(int64, tag = "3")]
            pub remain: i64,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerItemUsePayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<server_item_use_payload::ServerItemUseBody>,
}
/// Nested message and enum types in `ServerItemUsePayload`.
pub mod server_item_use_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerItemUseBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<server_item_use_body::ServerItemUseATar>,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerItemUseBody`.
    pub mod server_item_use_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerItemUseATar {
            #[prost(int64, tag = "1")]
            pub item_id: i64,
            #[prost(string, tag = "2")]
            pub item_name: ::prost::alloc::string::String,
            #[prost(int64, tag = "3")]
            pub num: i64,
            #[prost(int64, repeated, tag = "4")]
            pub position: ::prost::alloc::vec::Vec<i64>,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerGameGachaPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<server_game_gacha_payload::ServerGameGachaBody>,
}
/// Nested message and enum types in `ServerGameGachaPayload`.
pub mod server_game_gacha_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerGameGachaBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<server_game_gacha_body::ServerGameGachaATar>,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerGameGachaBody`.
    pub mod server_game_gacha_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerGameGachaATar {
            #[prost(int64, tag = "1")]
            pub gacha_id: i64,
            #[prost(string, tag = "2")]
            pub gacha_name: ::prost::alloc::string::String,
            #[prost(enumeration = "super::GachaType", tag = "3")]
            pub gacha_type: i32,
            #[prost(int64, repeated, tag = "4")]
            pub gacha_info: ::prost::alloc::vec::Vec<i64>,
        }
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum GachaType {
        InvalidGachaType = 0,
        DrawOne = 1,
        DrawTen = 2,
    }
    impl GachaType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                GachaType::InvalidGachaType => "InvalidGachaType",
                GachaType::DrawOne => "DrawOne",
                GachaType::DrawTen => "DrawTen",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "InvalidGachaType" => Some(Self::InvalidGachaType),
                "DrawOne" => Some(Self::DrawOne),
                "DrawTen" => Some(Self::DrawTen),
                _ => None,
            }
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerGameGachaPreferPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_game_gacha_prefer_payload::ServerGameGachaPreferBody,
    >,
}
/// Nested message and enum types in `ServerGameGachaPreferPayload`.
pub mod server_game_gacha_prefer_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerGameGachaPreferBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_game_gacha_prefer_body::ServerGameGachaPreferATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerGameGachaPreferBody`.
    pub mod server_game_gacha_prefer_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerGameGachaPreferATar {
            #[prost(int64, tag = "1")]
            pub prefer_id: i64,
            #[prost(int64, tag = "2")]
            pub odd_id: i64,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerBzFightPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<server_bz_fight_payload::ServerBzFightBody>,
}
/// Nested message and enum types in `ServerBzFightPayload`.
pub mod server_bz_fight_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerBzFightBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<server_bz_fight_body::ServerBzFightATar>,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerBzFightBody`.
    pub mod server_bz_fight_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerBzFightATar {
            #[prost(int64, tag = "1")]
            pub bz_id: i64,
            #[prost(string, tag = "2")]
            pub bz_name: ::prost::alloc::string::String,
            #[prost(int64, tag = "3")]
            pub stage_id: i64,
            #[prost(int64, tag = "4")]
            pub battle_id: i64,
            #[prost(message, optional, tag = "5")]
            pub team: ::core::option::Option<super::super::Team>,
            #[prost(enumeration = "super::BzFightResult", tag = "6")]
            pub result: i32,
            #[prost(message, repeated, tag = "7")]
            pub card_info: ::prost::alloc::vec::Vec<super::super::CardInfo>,
            #[prost(message, optional, tag = "8")]
            pub fight_info: ::core::option::Option<super::super::FightInfo>,
            #[prost(int64, repeated, tag = "9")]
            pub star_num: ::prost::alloc::vec::Vec<i64>,
            #[prost(int64, tag = "10")]
            pub duration: i64,
            #[prost(message, optional, tag = "11")]
            pub result_info: ::core::option::Option<super::super::FightResultInfo>,
        }
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum BzFightResult {
        InvalidBzResult = 0,
        BzResultSuccess = 1,
        BzResultFail = 2,
    }
    impl BzFightResult {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                BzFightResult::InvalidBzResult => "InvalidBzResult",
                BzFightResult::BzResultSuccess => "BzResultSuccess",
                BzFightResult::BzResultFail => "BzResultFail",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "InvalidBzResult" => Some(Self::InvalidBzResult),
                "BzResultSuccess" => Some(Self::BzResultSuccess),
                "BzResultFail" => Some(Self::BzResultFail),
                _ => None,
            }
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerBzChestsPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<server_bz_chests_payload::ServerBzChestsBody>,
}
/// Nested message and enum types in `ServerBzChestsPayload`.
pub mod server_bz_chests_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerBzChestsBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<server_bz_chests_body::ServerBzChestsATar>,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerBzChestsBody`.
    pub mod server_bz_chests_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerBzChestsATar {
            #[prost(int64, tag = "1")]
            pub bz_id: i64,
            #[prost(string, tag = "2")]
            pub bz_name: ::prost::alloc::string::String,
            #[prost(int64, tag = "3")]
            pub chests_id: i64,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerBzStagePayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<server_bz_stage_payload::ServerBzStageBody>,
}
/// Nested message and enum types in `ServerBzStagePayload`.
pub mod server_bz_stage_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerBzStageBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<server_bz_stage_body::ServerBzStageATar>,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerBzStageBody`.
    pub mod server_bz_stage_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerBzStageATar {
            #[prost(int64, tag = "1")]
            pub bz_id: i64,
            #[prost(string, tag = "2")]
            pub bz_name: ::prost::alloc::string::String,
            #[prost(int64, tag = "3")]
            pub stage_level: i64,
            #[prost(int64, tag = "4")]
            pub required_star_count: i64,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerShopBuyPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<server_shop_buy_payload::ServerShopBuyBody>,
}
/// Nested message and enum types in `ServerShopBuyPayload`.
pub mod server_shop_buy_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerShopBuyBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<server_shop_buy_body::ServerShopBuyATar>,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerShopBuyBody`.
    pub mod server_shop_buy_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerShopBuyATar {
            #[prost(int64, tag = "1")]
            pub shop_id: i64,
            #[prost(string, tag = "2")]
            pub shop_name: ::prost::alloc::string::String,
            #[prost(message, optional, tag = "3")]
            pub goods_info: ::core::option::Option<super::super::GoodsInfo>,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerMonthlyPassPurchasePayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_monthly_pass_purchase_payload::ServerMonthlyPassPurchaseBody,
    >,
}
/// Nested message and enum types in `ServerMonthlyPassPurchasePayload`.
pub mod server_monthly_pass_purchase_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerMonthlyPassPurchaseBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_monthly_pass_purchase_body::ServerMonthlyPassPurchaseATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerMonthlyPassPurchaseBody`.
    pub mod server_monthly_pass_purchase_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerMonthlyPassPurchaseATar {
            #[prost(string, tag = "1")]
            pub expire_at: ::prost::alloc::string::String,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerMonthlyPassRewardsPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_monthly_pass_rewards_payload::ServerMonthlyPassRewardsBody,
    >,
}
/// Nested message and enum types in `ServerMonthlyPassRewardsPayload`.
pub mod server_monthly_pass_rewards_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerMonthlyPassRewardsBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_monthly_pass_rewards_body::ServerMonthlyPassRewardsATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerMonthlyPassRewardsBody`.
    pub mod server_monthly_pass_rewards_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerMonthlyPassRewardsATar {
            #[prost(message, repeated, tag = "1")]
            pub reward_info: ::prost::alloc::vec::Vec<super::super::ItemInfo>,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerGameBillCreatePayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_game_bill_create_payload::ServerGameBillCreateBody,
    >,
}
/// Nested message and enum types in `ServerGameBillCreatePayload`.
pub mod server_game_bill_create_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerGameBillCreateBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_game_bill_create_body::ServerGameBillCreateATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerGameBillCreateBody`.
    pub mod server_game_bill_create_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerGameBillCreateATar {
            #[prost(string, tag = "1")]
            pub bill_id: ::prost::alloc::string::String,
            #[prost(string, tag = "2")]
            pub baas_id: ::prost::alloc::string::String,
            #[prost(int64, tag = "3")]
            pub shop_product_id: i64,
            #[prost(int64, tag = "4")]
            pub status: i64,
            #[prost(message, optional, tag = "5")]
            pub goods_info: ::core::option::Option<super::super::GoodsInfo>,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerGameBillClaimPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_game_bill_claim_payload::ServerGameBillClaimBody,
    >,
}
/// Nested message and enum types in `ServerGameBillClaimPayload`.
pub mod server_game_bill_claim_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerGameBillClaimBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_game_bill_claim_body::ServerGameBillClaimATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerGameBillClaimBody`.
    pub mod server_game_bill_claim_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerGameBillClaimATar {
            #[prost(string, tag = "1")]
            pub bill_id: ::prost::alloc::string::String,
            #[prost(string, tag = "2")]
            pub baas_id: ::prost::alloc::string::String,
            #[prost(int64, tag = "3")]
            pub shop_product_id: i64,
            #[prost(int64, tag = "4")]
            pub status: i64,
            #[prost(message, optional, tag = "5")]
            pub goods_info: ::core::option::Option<super::super::GoodsInfo>,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerGameBillFinishPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_game_bill_finish_payload::ServerGameBillFinishBody,
    >,
}
/// Nested message and enum types in `ServerGameBillFinishPayload`.
pub mod server_game_bill_finish_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerGameBillFinishBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_game_bill_finish_body::ServerGameBillFinishATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerGameBillFinishBody`.
    pub mod server_game_bill_finish_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerGameBillFinishATar {
            #[prost(string, tag = "1")]
            pub bill_id: ::prost::alloc::string::String,
            #[prost(string, tag = "2")]
            pub baas_id: ::prost::alloc::string::String,
            #[prost(int64, tag = "3")]
            pub shop_product_id: i64,
            #[prost(int64, tag = "4")]
            pub status: i64,
            #[prost(message, optional, tag = "5")]
            pub goods_info: ::core::option::Option<super::super::GoodsInfo>,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerGameBillCancelPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_game_bill_cancel_payload::ServerGameBillCancelBody,
    >,
}
/// Nested message and enum types in `ServerGameBillCancelPayload`.
pub mod server_game_bill_cancel_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerGameBillCancelBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_game_bill_cancel_body::ServerGameBillCancelATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerGameBillCancelBody`.
    pub mod server_game_bill_cancel_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerGameBillCancelATar {
            #[prost(string, tag = "1")]
            pub bill_id: ::prost::alloc::string::String,
            #[prost(string, tag = "2")]
            pub baas_id: ::prost::alloc::string::String,
            #[prost(int64, tag = "3")]
            pub shop_product_id: i64,
            #[prost(int64, tag = "4")]
            pub status: i64,
            #[prost(message, optional, tag = "5")]
            pub goods_info: ::core::option::Option<super::super::GoodsInfo>,
            #[prost(string, tag = "6")]
            pub reason: ::prost::alloc::string::String,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerBattlePassPurchasePayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_battle_pass_purchase_payload::ServerBattlePassPurchaseBody,
    >,
}
/// Nested message and enum types in `ServerBattlePassPurchasePayload`.
pub mod server_battle_pass_purchase_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerBattlePassPurchaseBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_battle_pass_purchase_body::ServerBattlePassPurchaseATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerBattlePassPurchaseBody`.
    pub mod server_battle_pass_purchase_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerBattlePassPurchaseATar {
            #[prost(int64, tag = "1")]
            pub season_id: i64,
            #[prost(int64, tag = "2")]
            pub stage_level: i64,
            #[prost(int64, tag = "3")]
            pub exp: i64,
            #[prost(int64, tag = "4")]
            pub level: i64,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerBattlePassLevelPurchasePayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_battle_pass_level_purchase_payload::ServerBattlePassLevelPurchaseBody,
    >,
}
/// Nested message and enum types in `ServerBattlePassLevelPurchasePayload`.
pub mod server_battle_pass_level_purchase_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerBattlePassLevelPurchaseBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_battle_pass_level_purchase_body::ServerBattlePassLevelPurchaseATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerBattlePassLevelPurchaseBody`.
    pub mod server_battle_pass_level_purchase_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerBattlePassLevelPurchaseATar {
            #[prost(int64, tag = "1")]
            pub season_id: i64,
            #[prost(int64, tag = "2")]
            pub stage_level: i64,
            #[prost(int64, tag = "3")]
            pub before_purchase_exp: i64,
            #[prost(int64, tag = "4")]
            pub before_purchase_level: i64,
            #[prost(int64, tag = "5")]
            pub level_count: i64,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerBattlePassRewardsPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_battle_pass_rewards_payload::ServerBattlePassRewardsBody,
    >,
}
/// Nested message and enum types in `ServerBattlePassRewardsPayload`.
pub mod server_battle_pass_rewards_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerBattlePassRewardsBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_battle_pass_rewards_body::ServerBattlePassRewardsATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerBattlePassRewardsBody`.
    pub mod server_battle_pass_rewards_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerBattlePassRewardsATar {
            #[prost(int64, tag = "1")]
            pub season_id: i64,
            #[prost(int64, tag = "2")]
            pub stage_level: i64,
            #[prost(int64, repeated, tag = "3")]
            pub levels: ::prost::alloc::vec::Vec<i64>,
            #[prost(int64, repeated, tag = "4")]
            pub level_boxes: ::prost::alloc::vec::Vec<i64>,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerBattlePassAllRewardsPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_battle_pass_all_rewards_payload::ServerBattlePassAllRewardsBody,
    >,
}
/// Nested message and enum types in `ServerBattlePassAllRewardsPayload`.
pub mod server_battle_pass_all_rewards_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerBattlePassAllRewardsBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_battle_pass_all_rewards_body::ServerBattlePassAllRewardsATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerBattlePassAllRewardsBody`.
    pub mod server_battle_pass_all_rewards_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerBattlePassAllRewardsATar {
            #[prost(int64, tag = "1")]
            pub season_id: i64,
            #[prost(int64, tag = "2")]
            pub stage_level: i64,
            #[prost(int64, repeated, tag = "4")]
            pub level_boxes: ::prost::alloc::vec::Vec<i64>,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerGemToStonePayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<server_gem_to_stone_payload::ServerGemToStoneBody>,
}
/// Nested message and enum types in `ServerGemToStonePayload`.
pub mod server_gem_to_stone_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerGemToStoneBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_gem_to_stone_body::ServerGemToStoneATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerGemToStoneBody`.
    pub mod server_gem_to_stone_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerGemToStoneATar {}
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerFirstPurchaseFinishPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_first_purchase_finish_payload::ServerFirstPurchaseFinishBody,
    >,
}
/// Nested message and enum types in `ServerFirstPurchaseFinishPayload`.
pub mod server_first_purchase_finish_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerFirstPurchaseFinishBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_first_purchase_finish_body::ServerFirstPurchaseFinishATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerFirstPurchaseFinishBody`.
    pub mod server_first_purchase_finish_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerFirstPurchaseFinishATar {
            #[prost(int64, tag = "1")]
            pub product_id: i64,
            #[prost(string, tag = "2")]
            pub product_name: ::prost::alloc::string::String,
            #[prost(string, tag = "3")]
            pub lcx_product_id: ::prost::alloc::string::String,
            #[prost(int64, tag = "4")]
            pub price: i64,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerUndergroundFinishPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_underground_finish_payload::ServerUndergroundFinishBody,
    >,
}
/// Nested message and enum types in `ServerUndergroundFinishPayload`.
pub mod server_underground_finish_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerUndergroundFinishBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_underground_finish_body::ServerUndergroundFinishATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerUndergroundFinishBody`.
    pub mod server_underground_finish_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerUndergroundFinishATar {
            #[prost(int64, tag = "1")]
            pub event_id: i64,
            #[prost(string, tag = "2")]
            pub event_name: ::prost::alloc::string::String,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerUndergroundClaimPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_underground_claim_payload::ServerUndergroundClaimBody,
    >,
}
/// Nested message and enum types in `ServerUndergroundClaimPayload`.
pub mod server_underground_claim_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerUndergroundClaimBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_underground_claim_body::ServerUndergroundClaimATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerUndergroundClaimBody`.
    pub mod server_underground_claim_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerUndergroundClaimATar {
            #[prost(int64, tag = "1")]
            pub event_id: i64,
            #[prost(string, tag = "2")]
            pub event_name: ::prost::alloc::string::String,
            #[prost(string, tag = "3")]
            pub is_first: ::prost::alloc::string::String,
            #[prost(int64, tag = "4")]
            pub event_type: i64,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerUndergroundSetCaptainPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_underground_set_captain_payload::ServerServerUndergroundSetCaptainBody,
    >,
}
/// Nested message and enum types in `ServerUndergroundSetCaptainPayload`.
pub mod server_underground_set_captain_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerServerUndergroundSetCaptainBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_server_underground_set_captain_body::ServerServerUndergroundSetCaptainATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerServerUndergroundSetCaptainBody`.
    pub mod server_server_underground_set_captain_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerServerUndergroundSetCaptainATar {
            #[prost(int64, tag = "1")]
            pub card_id: i64,
            #[prost(string, tag = "2")]
            pub card_name: ::prost::alloc::string::String,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerUndergroundStartSectionPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_underground_start_section_payload::ServerUndergroundStartSectionBody,
    >,
}
/// Nested message and enum types in `ServerUndergroundStartSectionPayload`.
pub mod server_underground_start_section_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerUndergroundStartSectionBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_underground_start_section_body::ServerUndergroundStartSectionATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerUndergroundStartSectionBody`.
    pub mod server_underground_start_section_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerUndergroundStartSectionATar {
            #[prost(int64, tag = "1")]
            pub section_id: i64,
            #[prost(string, tag = "2")]
            pub section_name: ::prost::alloc::string::String,
            #[prost(int64, tag = "3")]
            pub stamina_cost: i64,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerUndergroundStartChapterPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_underground_start_chapter_payload::ServerUndergroundStartChapterBody,
    >,
}
/// Nested message and enum types in `ServerUndergroundStartChapterPayload`.
pub mod server_underground_start_chapter_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerUndergroundStartChapterBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_underground_start_chapter_body::ServerUndergroundStartChapterATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerUndergroundStartChapterBody`.
    pub mod server_underground_start_chapter_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerUndergroundStartChapterATar {
            #[prost(int64, tag = "1")]
            pub chapter_id: i64,
            #[prost(string, tag = "2")]
            pub chapter_name: ::prost::alloc::string::String,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerUndergroundUnlockSectionPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_underground_unlock_section_payload::ServerUndergroundUnlockSectionBody,
    >,
}
/// Nested message and enum types in `ServerUndergroundUnlockSectionPayload`.
pub mod server_underground_unlock_section_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerUndergroundUnlockSectionBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_underground_unlock_section_body::ServerUndergroundUnlockSectionATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerUndergroundUnlockSectionBody`.
    pub mod server_underground_unlock_section_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerUndergroundUnlockSectionATar {
            #[prost(int64, tag = "1")]
            pub section_id: i64,
            #[prost(string, tag = "2")]
            pub section_name: ::prost::alloc::string::String,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerUndergroundUnlockChapterPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_underground_unlock_chapter_payload::ServerUndergroundUnlockChapterBody,
    >,
}
/// Nested message and enum types in `ServerUndergroundUnlockChapterPayload`.
pub mod server_underground_unlock_chapter_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerUndergroundUnlockChapterBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_underground_unlock_chapter_body::ServerUndergroundUnlockChapterATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerUndergroundUnlockChapterBody`.
    pub mod server_underground_unlock_chapter_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerUndergroundUnlockChapterATar {
            #[prost(int64, tag = "1")]
            pub chapter_id: i64,
            #[prost(string, tag = "2")]
            pub chapter_name: ::prost::alloc::string::String,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerUndergroundStartBattlePayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_underground_start_battle_payload::ServerUndergroundStartBattleBody,
    >,
}
/// Nested message and enum types in `ServerUndergroundStartBattlePayload`.
pub mod server_underground_start_battle_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerUndergroundStartBattleBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_underground_start_battle_body::ServerUndergroundStartBattleATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerUndergroundStartBattleBody`.
    pub mod server_underground_start_battle_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerUndergroundStartBattleATar {
            #[prost(int64, tag = "1")]
            pub event_id: i64,
            #[prost(string, tag = "2")]
            pub event_name: ::prost::alloc::string::String,
            #[prost(int64, tag = "3")]
            pub battle_stage_id: i64,
            #[prost(string, tag = "4")]
            pub battle_unique_id: ::prost::alloc::string::String,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerUndergroundFinishBattlePayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_underground_finish_battle_payload::ServerUndergroundFinishBattleBody,
    >,
}
/// Nested message and enum types in `ServerUndergroundFinishBattlePayload`.
pub mod server_underground_finish_battle_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerUndergroundFinishBattleBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_underground_finish_battle_body::ServerUndergroundFinishBattleATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerUndergroundFinishBattleBody`.
    pub mod server_underground_finish_battle_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerUndergroundFinishBattleATar {
            #[prost(int64, tag = "1")]
            pub event_id: i64,
            #[prost(string, tag = "2")]
            pub event_name: ::prost::alloc::string::String,
            #[prost(string, tag = "3")]
            pub is_first: ::prost::alloc::string::String,
            #[prost(int64, tag = "4")]
            pub battle_stage_id: i64,
            #[prost(message, optional, tag = "5")]
            pub fight_info: ::core::option::Option<super::super::FightInfo>,
            #[prost(message, optional, tag = "6")]
            pub team: ::core::option::Option<super::super::Team>,
            #[prost(int64, tag = "7")]
            pub time: i64,
            #[prost(int64, tag = "8")]
            pub result: i64,
            #[prost(int64, repeated, tag = "9")]
            pub star: ::prost::alloc::vec::Vec<i64>,
            #[prost(string, tag = "10")]
            pub is_first_three_star: ::prost::alloc::string::String,
            #[prost(string, tag = "11")]
            pub battle_unique_id: ::prost::alloc::string::String,
            #[prost(message, repeated, tag = "12")]
            pub card_info: ::prost::alloc::vec::Vec<super::super::CardInfo>,
            #[prost(message, repeated, tag = "13")]
            pub equip_info: ::prost::alloc::vec::Vec<super::super::PlayerEquipInfo>,
            #[prost(message, repeated, tag = "14")]
            pub phoneme_info: ::prost::alloc::vec::Vec<super::super::PlayerPhonemeInfo>,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerCrusadeChoosePayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_crusade_choose_payload::ServerCrusadeChooseBody,
    >,
}
/// Nested message and enum types in `ServerCrusadeChoosePayload`.
pub mod server_crusade_choose_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerCrusadeChooseBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_crusade_choose_body::ServerCrusadeChooseATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerCrusadeChooseBody`.
    pub mod server_crusade_choose_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerCrusadeChooseATar {
            #[prost(int64, tag = "1")]
            pub choose_level_master_id: i64,
            #[prost(int64, tag = "2")]
            pub env_id: i64,
            #[prost(int64, tag = "3")]
            pub map_id: i64,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerCrusadeSetDeckPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_crusade_set_deck_payload::ServerCrusadeSetDeckBody,
    >,
}
/// Nested message and enum types in `ServerCrusadeSetDeckPayload`.
pub mod server_crusade_set_deck_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerCrusadeSetDeckBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_crusade_set_deck_body::ServerCrusadeSetDeckATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerCrusadeSetDeckBody`.
    pub mod server_crusade_set_deck_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerCrusadeSetDeckATar {
            #[prost(int64, tag = "1")]
            pub choose_level_master_id: i64,
            #[prost(int64, tag = "2")]
            pub env_id: i64,
            #[prost(int64, tag = "3")]
            pub map_id: i64,
            #[prost(int64, repeated, tag = "4")]
            pub character_master_ids: ::prost::alloc::vec::Vec<i64>,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerCrusadeMovePayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<server_crusade_move_payload::ServerCrusadeMoveBody>,
}
/// Nested message and enum types in `ServerCrusadeMovePayload`.
pub mod server_crusade_move_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerCrusadeMoveBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_crusade_move_body::ServerCrusadeMoveATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerCrusadeMoveBody`.
    pub mod server_crusade_move_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerCrusadeMoveATar {
            #[prost(int64, tag = "1")]
            pub previous_point_id: i64,
            #[prost(int64, tag = "2")]
            pub arrive_point_id: i64,
            #[prost(int64, tag = "3")]
            pub event_id: i64,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerCrusadeEventStartPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_crusade_event_start_payload::ServerCrusadeEventStartBody,
    >,
}
/// Nested message and enum types in `ServerCrusadeEventStartPayload`.
pub mod server_crusade_event_start_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerCrusadeEventStartBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_crusade_event_start_body::ServerCrusadeEventStartATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerCrusadeEventStartBody`.
    pub mod server_crusade_event_start_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerCrusadeEventStartATar {
            #[prost(int64, tag = "1")]
            pub event_id: i64,
            #[prost(int64, tag = "2")]
            pub event_type: i64,
            #[prost(int64, tag = "3")]
            pub node_id: i64,
            #[prost(int64, repeated, tag = "4")]
            pub treasure: ::prost::alloc::vec::Vec<i64>,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerCrusadeEventFinishPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_crusade_event_finish_payload::ServerCrusadeEventFinishBody,
    >,
}
/// Nested message and enum types in `ServerCrusadeEventFinishPayload`.
pub mod server_crusade_event_finish_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerCrusadeEventFinishBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_crusade_event_finish_body::ServerCrusadeEventFinishATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerCrusadeEventFinishBody`.
    pub mod server_crusade_event_finish_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerCrusadeEventFinishATar {
            #[prost(int64, tag = "1")]
            pub event_id: i64,
            #[prost(int64, tag = "2")]
            pub option_id: i64,
            #[prost(int64, repeated, tag = "3")]
            pub option_result_id: ::prost::alloc::vec::Vec<i64>,
            #[prost(int64, tag = "4")]
            pub character_id: i64,
            #[prost(int64, tag = "5")]
            pub coin: i64,
            #[prost(int64, repeated, tag = "6")]
            pub treasure: ::prost::alloc::vec::Vec<i64>,
            #[prost(enumeration = "super::EventResult", tag = "7")]
            pub event_result: i32,
            #[prost(string, tag = "8")]
            pub is_special: ::prost::alloc::string::String,
        }
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum EventResult {
        End = 0,
        Success = 1,
        Fail = 2,
    }
    impl EventResult {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                EventResult::End => "End",
                EventResult::Success => "Success",
                EventResult::Fail => "Fail",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "End" => Some(Self::End),
                "Success" => Some(Self::Success),
                "Fail" => Some(Self::Fail),
                _ => None,
            }
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerCrusadeRestartPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_crusade_restart_payload::ServerCrusadeRestartBody,
    >,
}
/// Nested message and enum types in `ServerCrusadeRestartPayload`.
pub mod server_crusade_restart_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerCrusadeRestartBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_crusade_restart_body::ServerCrusadeRestartATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerCrusadeRestartBody`.
    pub mod server_crusade_restart_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerCrusadeRestartATar {
            #[prost(int64, tag = "1")]
            pub choose_level_master_id: i64,
            #[prost(int64, tag = "2")]
            pub env_id: i64,
            #[prost(int64, tag = "3")]
            pub map_id: i64,
            #[prost(int64, repeated, tag = "4")]
            pub event_routes: ::prost::alloc::vec::Vec<i64>,
            #[prost(int64, repeated, tag = "5")]
            pub node_routes: ::prost::alloc::vec::Vec<i64>,
            #[prost(int64, repeated, tag = "6")]
            pub supports: ::prost::alloc::vec::Vec<i64>,
            #[prost(int64, repeated, tag = "7")]
            pub treasure: ::prost::alloc::vec::Vec<i64>,
            #[prost(int64, tag = "8")]
            pub remain_times: i64,
            #[prost(message, repeated, tag = "9")]
            pub reward: ::prost::alloc::vec::Vec<super::super::ItemInfo>,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerCrusadeFinishPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_crusade_finish_payload::ServerCrusadeFinishBody,
    >,
}
/// Nested message and enum types in `ServerCrusadeFinishPayload`.
pub mod server_crusade_finish_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerCrusadeFinishBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_crusade_finish_body::ServerCrusadeFinishATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerCrusadeFinishBody`.
    pub mod server_crusade_finish_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerCrusadeFinishATar {
            #[prost(int64, tag = "1")]
            pub choose_level_master_id: i64,
            #[prost(int64, tag = "2")]
            pub env_id: i64,
            #[prost(int64, tag = "3")]
            pub map_id: i64,
            #[prost(int64, repeated, tag = "4")]
            pub event_routes: ::prost::alloc::vec::Vec<i64>,
            #[prost(int64, repeated, tag = "5")]
            pub node_routes: ::prost::alloc::vec::Vec<i64>,
            #[prost(int64, repeated, tag = "6")]
            pub supports: ::prost::alloc::vec::Vec<i64>,
            #[prost(int64, repeated, tag = "7")]
            pub treasure: ::prost::alloc::vec::Vec<i64>,
            #[prost(int64, tag = "8")]
            pub remain_times: i64,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerCrusadeBattleStartPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_crusade_battle_start_payload::ServerCrusadeBattleStartBody,
    >,
}
/// Nested message and enum types in `ServerCrusadeBattleStartPayload`.
pub mod server_crusade_battle_start_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerCrusadeBattleStartBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_crusade_battle_start_body::ServerCrusadeBattleStartATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerCrusadeBattleStartBody`.
    pub mod server_crusade_battle_start_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerCrusadeBattleStartATar {
            #[prost(int64, tag = "1")]
            pub event_id: i64,
            #[prost(string, tag = "2")]
            pub event_name: ::prost::alloc::string::String,
            #[prost(string, tag = "3")]
            pub is_first: ::prost::alloc::string::String,
            #[prost(int64, tag = "4")]
            pub battle_stage_id: i64,
            #[prost(string, tag = "5")]
            pub battle_unique_id: ::prost::alloc::string::String,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerCrusadeBattleFinishPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_crusade_battle_finish_payload::ServerCrusadeBattleFinishBody,
    >,
}
/// Nested message and enum types in `ServerCrusadeBattleFinishPayload`.
pub mod server_crusade_battle_finish_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerCrusadeBattleFinishBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_crusade_battle_finish_body::ServerCrusadeBattleFinishATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerCrusadeBattleFinishBody`.
    pub mod server_crusade_battle_finish_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerCrusadeBattleFinishATar {
            #[prost(int64, tag = "1")]
            pub event_id: i64,
            #[prost(string, tag = "2")]
            pub event_name: ::prost::alloc::string::String,
            #[prost(int64, tag = "21")]
            pub crusade_type_id: i64,
            #[prost(string, tag = "3")]
            pub is_first: ::prost::alloc::string::String,
            #[prost(int64, tag = "4")]
            pub battle_stage_id: i64,
            #[prost(message, optional, tag = "5")]
            pub fight_info: ::core::option::Option<super::super::FightInfo>,
            #[prost(int64, tag = "6")]
            pub skip: i64,
            #[prost(int64, tag = "7")]
            pub time: i64,
            #[prost(int64, tag = "8")]
            pub stamina_cost: i64,
            #[prost(message, optional, tag = "9")]
            pub result_info: ::core::option::Option<super::super::FightResultInfo>,
            #[prost(int64, tag = "10")]
            pub result: i64,
            #[prost(message, optional, tag = "11")]
            pub team: ::core::option::Option<super::super::Team>,
            #[prost(string, tag = "12")]
            pub battle_unique_id: ::prost::alloc::string::String,
            #[prost(message, repeated, tag = "13")]
            pub card_info: ::prost::alloc::vec::Vec<super::super::CardInfo>,
            #[prost(message, repeated, tag = "14")]
            pub equip_info: ::prost::alloc::vec::Vec<super::super::PlayerEquipInfo>,
            #[prost(message, repeated, tag = "15")]
            pub phoneme_info: ::prost::alloc::vec::Vec<super::super::PlayerPhonemeInfo>,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerCrusadeActivityEnterPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_crusade_activity_enter_payload::ServerCrusadeActivityEnterBody,
    >,
}
/// Nested message and enum types in `ServerCrusadeActivityEnterPayload`.
pub mod server_crusade_activity_enter_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerCrusadeActivityEnterBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_crusade_activity_enter_body::ServerCrusadeActivityEnterATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerCrusadeActivityEnterBody`.
    pub mod server_crusade_activity_enter_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerCrusadeActivityEnterATar {
            #[prost(int64, tag = "1")]
            pub type_master_id: i64,
            #[prost(int64, tag = "2")]
            pub env_id: i64,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerCrusadeActivityArriveNodePayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_crusade_activity_arrive_node_payload::ServerCrusadeActivityArriveNodeBody,
    >,
}
/// Nested message and enum types in `ServerCrusadeActivityArriveNodePayload`.
pub mod server_crusade_activity_arrive_node_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerCrusadeActivityArriveNodeBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_crusade_activity_arrive_node_body::ServerCrusadeActivityArriveNodeATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerCrusadeActivityArriveNodeBody`.
    pub mod server_crusade_activity_arrive_node_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerCrusadeActivityArriveNodeATar {
            #[prost(int64, tag = "1")]
            pub type_master_id: i64,
            #[prost(int64, tag = "2")]
            pub node_id: i64,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerCrusadeActivityCollectionRewardsPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_crusade_activity_collection_rewards_payload::ServerCrusadeActivityCollectionRewardsBody,
    >,
}
/// Nested message and enum types in `ServerCrusadeActivityCollectionRewardsPayload`.
pub mod server_crusade_activity_collection_rewards_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerCrusadeActivityCollectionRewardsBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_crusade_activity_collection_rewards_body::ServerCrusadeActivityCollectionRewardsATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerCrusadeActivityCollectionRewardsBody`.
    pub mod server_crusade_activity_collection_rewards_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerCrusadeActivityCollectionRewardsATar {
            #[prost(int64, tag = "1")]
            pub type_master_id: i64,
            #[prost(int64, tag = "2")]
            pub collections_master_id: i64,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerCrusadeActivityEventStartPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_crusade_activity_event_start_payload::ServerCrusadeActivityEventStartBody,
    >,
}
/// Nested message and enum types in `ServerCrusadeActivityEventStartPayload`.
pub mod server_crusade_activity_event_start_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerCrusadeActivityEventStartBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_crusade_activity_event_start_body::ServerCrusadeActivityEventStartATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerCrusadeActivityEventStartBody`.
    pub mod server_crusade_activity_event_start_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerCrusadeActivityEventStartATar {
            #[prost(int64, tag = "1")]
            pub event_id: i64,
            #[prost(int64, tag = "2")]
            pub event_type: i64,
            #[prost(string, tag = "3")]
            pub is_first: ::prost::alloc::string::String,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerCrusadeActivityEventFinishPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_crusade_activity_event_finish_payload::ServerCrusadeActivityEventFinishBody,
    >,
}
/// Nested message and enum types in `ServerCrusadeActivityEventFinishPayload`.
pub mod server_crusade_activity_event_finish_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerCrusadeActivityEventFinishBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_crusade_activity_event_finish_body::ServerCrusadeActivityEventFinishATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerCrusadeActivityEventFinishBody`.
    pub mod server_crusade_activity_event_finish_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerCrusadeActivityEventFinishATar {
            #[prost(int64, tag = "1")]
            pub event_id: i64,
            #[prost(int64, tag = "2")]
            pub event_type: i64,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerCrusadeActivityUndergroundStartPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_crusade_activity_underground_start_payload::ServerCrusadeActivityUndergroundStartBody,
    >,
}
/// Nested message and enum types in `ServerCrusadeActivityUndergroundStartPayload`.
pub mod server_crusade_activity_underground_start_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerCrusadeActivityUndergroundStartBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_crusade_activity_underground_start_body::ServerCrusadeActivityUndergroundStartATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerCrusadeActivityUndergroundStartBody`.
    pub mod server_crusade_activity_underground_start_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerCrusadeActivityUndergroundStartATar {
            #[prost(int64, tag = "1")]
            pub type_master_id: i64,
            #[prost(int64, tag = "2")]
            pub node_id: i64,
            #[prost(int64, tag = "3")]
            pub underground_id: i64,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerCrusadeActivityUndergroundFinishPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_crusade_activity_underground_finish_payload::ServerCrusadeActivityUndergroundFinishBody,
    >,
}
/// Nested message and enum types in `ServerCrusadeActivityUndergroundFinishPayload`.
pub mod server_crusade_activity_underground_finish_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerCrusadeActivityUndergroundFinishBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_crusade_activity_underground_finish_body::ServerCrusadeActivityUndergroundFinishATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerCrusadeActivityUndergroundFinishBody`.
    pub mod server_crusade_activity_underground_finish_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerCrusadeActivityUndergroundFinishATar {
            #[prost(int64, tag = "1")]
            pub type_master_id: i64,
            #[prost(int64, tag = "2")]
            pub node_id: i64,
            #[prost(int64, tag = "3")]
            pub underground_id: i64,
            #[prost(string, tag = "4")]
            pub win: ::prost::alloc::string::String,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerDeviceBindPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<server_device_bind_payload::ServerDeviceBindBody>,
}
/// Nested message and enum types in `ServerDeviceBindPayload`.
pub mod server_device_bind_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerDeviceBindBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<server_device_bind_body::ServerDeviceBindATar>,
    }
    /// Nested message and enum types in `ServerDeviceBindBody`.
    pub mod server_device_bind_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerDeviceBindATar {
            #[prost(string, tag = "1")]
            pub player_id: ::prost::alloc::string::String,
            #[prost(string, tag = "2")]
            pub device_id: ::prost::alloc::string::String,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerDisguisePurchasePayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_disguise_purchase_payload::ServerDisguisePurchaseBody,
    >,
}
/// Nested message and enum types in `ServerDisguisePurchasePayload`.
pub mod server_disguise_purchase_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerDisguisePurchaseBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_disguise_purchase_body::ServerDisguisePurchaseATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerDisguisePurchaseBody`.
    pub mod server_disguise_purchase_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerDisguisePurchaseATar {
            #[prost(int64, tag = "1")]
            pub outfit_id: i64,
            #[prost(string, tag = "2")]
            pub outfit_name: ::prost::alloc::string::String,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerDisguiseFitPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<server_disguise_fit_payload::ServerDisguiseFitBody>,
}
/// Nested message and enum types in `ServerDisguiseFitPayload`.
pub mod server_disguise_fit_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerDisguiseFitBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_disguise_fit_body::ServerDisguiseFitATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerDisguiseFitBody`.
    pub mod server_disguise_fit_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerDisguiseFitATar {
            #[prost(int64, tag = "1")]
            pub outfit_id: i64,
            #[prost(string, tag = "2")]
            pub outfit_name: ::prost::alloc::string::String,
            #[prost(string, tag = "3")]
            pub target: ::prost::alloc::string::String,
            #[prost(int64, tag = "4")]
            pub equip: i64,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerBattleActivityFightPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_battle_activity_fight_payload::ServerBattleActivityFightBody,
    >,
}
/// Nested message and enum types in `ServerBattleActivityFightPayload`.
pub mod server_battle_activity_fight_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerBattleActivityFightBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_battle_activity_fight_body::ServerBattleActivityFightATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerBattleActivityFightBody`.
    pub mod server_battle_activity_fight_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerBattleActivityFightATar {
            #[prost(int64, tag = "1")]
            pub battle_activity_battle_id: i64,
            #[prost(string, tag = "2")]
            pub battle_activity_name: ::prost::alloc::string::String,
            #[prost(message, optional, tag = "3")]
            pub team: ::core::option::Option<super::super::Team>,
            #[prost(enumeration = "super::BattleActivityResult", tag = "4")]
            pub result: i32,
            #[prost(message, repeated, tag = "5")]
            pub card_info: ::prost::alloc::vec::Vec<super::super::CardInfo>,
            #[prost(message, optional, tag = "6")]
            pub fight_info: ::core::option::Option<super::super::FightInfo>,
            #[prost(message, optional, tag = "7")]
            pub result_info: ::core::option::Option<super::super::FightResultInfo>,
            #[prost(int64, tag = "8")]
            pub time: i64,
            #[prost(int64, tag = "9")]
            pub stamina_cost: i64,
            #[prost(message, repeated, tag = "10")]
            pub first_clear_reward: ::prost::alloc::vec::Vec<
                server_battle_activity_fight_a_tar::FirstClearRewardEntry,
            >,
            #[prost(message, repeated, tag = "11")]
            pub reward: ::prost::alloc::vec::Vec<
                server_battle_activity_fight_a_tar::RewardEntry,
            >,
            #[prost(message, repeated, tag = "12")]
            pub equip_info: ::prost::alloc::vec::Vec<super::super::PlayerEquipInfo>,
            #[prost(message, repeated, tag = "13")]
            pub phoneme_info: ::prost::alloc::vec::Vec<super::super::PlayerPhonemeInfo>,
        }
        /// Nested message and enum types in `ServerBattleActivityFightATar`.
        pub mod server_battle_activity_fight_a_tar {
            #[derive(serde::Serialize, serde::Deserialize)]
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct FirstClearRewardEntry {
                #[prost(int64, tag = "1")]
                pub key: i64,
                #[prost(int64, tag = "2")]
                pub value: i64,
            }
            #[derive(serde::Serialize, serde::Deserialize)]
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct RewardEntry {
                #[prost(int64, tag = "1")]
                pub key: i64,
                #[prost(int64, tag = "2")]
                pub value: i64,
            }
        }
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum BattleActivityResult {
        InvalidDqResult = 0,
        DqResultSuccess = 1,
        DqResultFail = 2,
    }
    impl BattleActivityResult {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                BattleActivityResult::InvalidDqResult => "InvalidDqResult",
                BattleActivityResult::DqResultSuccess => "DqResultSuccess",
                BattleActivityResult::DqResultFail => "DqResultFail",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "InvalidDqResult" => Some(Self::InvalidDqResult),
                "DqResultSuccess" => Some(Self::DqResultSuccess),
                "DqResultFail" => Some(Self::DqResultFail),
                _ => None,
            }
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerBattleActivityFightStartPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_battle_activity_fight_start_payload::ServerBattleActivityFightStartBody,
    >,
}
/// Nested message and enum types in `ServerBattleActivityFightStartPayload`.
pub mod server_battle_activity_fight_start_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerBattleActivityFightStartBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_battle_activity_fight_start_body::ServerBattleActivityFightStartATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerBattleActivityFightStartBody`.
    pub mod server_battle_activity_fight_start_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerBattleActivityFightStartATar {
            #[prost(int64, tag = "1")]
            pub battle_activity_battle_id: i64,
            #[prost(string, tag = "2")]
            pub battle_activity_name: ::prost::alloc::string::String,
            #[prost(string, tag = "3")]
            pub battle_unique_id: ::prost::alloc::string::String,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerBattleActivityFightFinishPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_battle_activity_fight_finish_payload::ServerBattleActivityFightFinishBody,
    >,
}
/// Nested message and enum types in `ServerBattleActivityFightFinishPayload`.
pub mod server_battle_activity_fight_finish_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerBattleActivityFightFinishBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_battle_activity_fight_finish_body::ServerBattleActivityFightFinishATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerBattleActivityFightFinishBody`.
    pub mod server_battle_activity_fight_finish_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerBattleActivityFightFinishATar {
            #[prost(int64, tag = "1")]
            pub battle_activity_battle_id: i64,
            #[prost(string, tag = "2")]
            pub battle_activity_name: ::prost::alloc::string::String,
            #[prost(message, optional, tag = "3")]
            pub team: ::core::option::Option<super::super::Team>,
            #[prost(enumeration = "super::BattleActivityResult", tag = "4")]
            pub result: i32,
            #[prost(message, repeated, tag = "5")]
            pub card_info: ::prost::alloc::vec::Vec<super::super::CardInfo>,
            #[prost(message, optional, tag = "6")]
            pub fight_info: ::core::option::Option<super::super::FightInfo>,
            #[prost(message, optional, tag = "7")]
            pub result_info: ::core::option::Option<super::super::FightResultInfo>,
            #[prost(int64, tag = "8")]
            pub time: i64,
            #[prost(int64, tag = "9")]
            pub stamina_cost: i64,
            #[prost(message, repeated, tag = "10")]
            pub first_clear_reward: ::prost::alloc::vec::Vec<
                server_battle_activity_fight_finish_a_tar::FirstClearRewardEntry,
            >,
            #[prost(message, repeated, tag = "11")]
            pub reward: ::prost::alloc::vec::Vec<
                server_battle_activity_fight_finish_a_tar::RewardEntry,
            >,
            #[prost(string, tag = "12")]
            pub battle_unique_id: ::prost::alloc::string::String,
            #[prost(message, repeated, tag = "13")]
            pub equip_info: ::prost::alloc::vec::Vec<super::super::PlayerEquipInfo>,
            #[prost(message, repeated, tag = "14")]
            pub phoneme_info: ::prost::alloc::vec::Vec<super::super::PlayerPhonemeInfo>,
        }
        /// Nested message and enum types in `ServerBattleActivityFightFinishATar`.
        pub mod server_battle_activity_fight_finish_a_tar {
            #[derive(serde::Serialize, serde::Deserialize)]
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct FirstClearRewardEntry {
                #[prost(int64, tag = "1")]
                pub key: i64,
                #[prost(int64, tag = "2")]
                pub value: i64,
            }
            #[derive(serde::Serialize, serde::Deserialize)]
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct RewardEntry {
                #[prost(int64, tag = "1")]
                pub key: i64,
                #[prost(int64, tag = "2")]
                pub value: i64,
            }
        }
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum BattleActivityResult {
        InvalidDqResult = 0,
        DqResultSuccess = 1,
        DqResultFail = 2,
    }
    impl BattleActivityResult {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                BattleActivityResult::InvalidDqResult => "InvalidDqResult",
                BattleActivityResult::DqResultSuccess => "DqResultSuccess",
                BattleActivityResult::DqResultFail => "DqResultFail",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "InvalidDqResult" => Some(Self::InvalidDqResult),
                "DqResultSuccess" => Some(Self::DqResultSuccess),
                "DqResultFail" => Some(Self::DqResultFail),
                _ => None,
            }
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerBattleActivityPlotPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_battle_activity_plot_payload::ServerBattleActivityPlotBody,
    >,
}
/// Nested message and enum types in `ServerBattleActivityPlotPayload`.
pub mod server_battle_activity_plot_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerBattleActivityPlotBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_battle_activity_plot_body::ServerBattleActivityPlotATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerBattleActivityPlotBody`.
    pub mod server_battle_activity_plot_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerBattleActivityPlotATar {
            #[prost(int64, tag = "1")]
            pub battle_activity_master_id: i64,
            #[prost(int64, tag = "2")]
            pub battle_activity_plot_master_id: i64,
            #[prost(message, repeated, tag = "3")]
            pub rewards: ::prost::alloc::vec::Vec<
                server_battle_activity_plot_a_tar::RewardsEntry,
            >,
            #[prost(int64, tag = "4")]
            pub is_first_clear: i64,
        }
        /// Nested message and enum types in `ServerBattleActivityPlotATar`.
        pub mod server_battle_activity_plot_a_tar {
            #[derive(serde::Serialize, serde::Deserialize)]
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct RewardsEntry {
                #[prost(int64, tag = "1")]
                pub key: i64,
                #[prost(int64, tag = "2")]
                pub value: i64,
            }
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerBattleActivityHangStartPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_battle_activity_hang_start_payload::ServerBattleActivityBody,
    >,
}
/// Nested message and enum types in `ServerBattleActivityHangStartPayload`.
pub mod server_battle_activity_hang_start_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerBattleActivityBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_battle_activity_body::ServerBattleActivityATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerBattleActivityBody`.
    pub mod server_battle_activity_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerBattleActivityATar {
            #[prost(int64, tag = "1")]
            pub battle_activity_master_id: i64,
            #[prost(int64, tag = "2")]
            pub battle_activity_battle_master_id: i64,
            #[prost(int64, tag = "3")]
            pub times: i64,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerBattleActivityHangEndPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_battle_activity_hang_end_payload::ServerBattleActivityBody,
    >,
}
/// Nested message and enum types in `ServerBattleActivityHangEndPayload`.
pub mod server_battle_activity_hang_end_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerBattleActivityBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_battle_activity_body::ServerBattleActivityATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerBattleActivityBody`.
    pub mod server_battle_activity_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerBattleActivityATar {
            #[prost(int64, tag = "1")]
            pub battle_activity_master_id: i64,
            #[prost(int64, tag = "2")]
            pub battle_activity_battle_master_id: i64,
            #[prost(int64, tag = "3")]
            pub expire_times: i64,
            #[prost(int64, tag = "4")]
            pub times: i64,
            #[prost(message, repeated, tag = "5")]
            pub return_rewards: ::prost::alloc::vec::Vec<super::super::ItemInfo>,
            #[prost(int64, tag = "6")]
            pub status: i64,
            #[prost(int64, tag = "7")]
            pub duration: i64,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerMasterSectionBattleStartPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_master_section_battle_start_payload::ServerMasterSectionBattleStartBody,
    >,
}
/// Nested message and enum types in `ServerMasterSectionBattleStartPayload`.
pub mod server_master_section_battle_start_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerMasterSectionBattleStartBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_master_section_battle_start_body::ServerMasterSectionBattleStartATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerMasterSectionBattleStartBody`.
    pub mod server_master_section_battle_start_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerMasterSectionBattleStartATar {
            #[prost(int64, tag = "1")]
            pub master_section_id: i64,
            #[prost(int64, tag = "2")]
            pub stage_id: i64,
            #[prost(int64, repeated, tag = "3")]
            pub buff_choose: ::prost::alloc::vec::Vec<i64>,
            #[prost(int64, repeated, tag = "4")]
            pub master_condition: ::prost::alloc::vec::Vec<i64>,
            #[prost(int64, tag = "5")]
            pub master_level: i64,
            #[prost(string, tag = "6")]
            pub battle_unique_id: ::prost::alloc::string::String,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerMasterSectionBattleFinishPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_master_section_battle_finish_payload::ServerMasterSectionBattleFinishBody,
    >,
}
/// Nested message and enum types in `ServerMasterSectionBattleFinishPayload`.
pub mod server_master_section_battle_finish_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerMasterSectionBattleFinishBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_master_section_battle_finish_body::ServerMasterSectionBattleFinishATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerMasterSectionBattleFinishBody`.
    pub mod server_master_section_battle_finish_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerMasterSectionBattleFinishATar {
            #[prost(int64, tag = "1")]
            pub master_section_id: i64,
            #[prost(int64, tag = "2")]
            pub stage_id: i64,
            #[prost(int64, tag = "3")]
            pub master_level: i64,
            #[prost(string, tag = "4")]
            pub battle_unique_id: ::prost::alloc::string::String,
            #[prost(message, optional, tag = "5")]
            pub team: ::core::option::Option<super::super::Team>,
            #[prost(int64, tag = "6")]
            pub result: i64,
            #[prost(message, repeated, tag = "7")]
            pub card_info: ::prost::alloc::vec::Vec<super::super::CardInfo>,
            #[prost(message, repeated, tag = "8")]
            pub equip_info: ::prost::alloc::vec::Vec<super::super::PlayerEquipInfo>,
            #[prost(message, repeated, tag = "9")]
            pub phoneme_info: ::prost::alloc::vec::Vec<super::super::PlayerPhonemeInfo>,
            #[prost(message, optional, tag = "10")]
            pub fight_info: ::core::option::Option<super::super::FightInfo>,
            #[prost(message, optional, tag = "11")]
            pub result_info: ::core::option::Option<super::super::FightResultInfo>,
            #[prost(string, tag = "12")]
            pub is_first: ::prost::alloc::string::String,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerMasterSectionLevelUpPayload {
    #[prost(string, tag = "1")]
    pub log_store: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub request_stamp: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub log_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub app_id: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub lcx_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub role_info: ::core::option::Option<RoleInfo>,
    #[prost(int64, tag = "7")]
    pub event_time: i64,
    #[prost(string, tag = "8")]
    pub event_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "11")]
    pub body: ::core::option::Option<
        server_master_section_level_up_payload::ServerMasterSectionLevelUpBody,
    >,
}
/// Nested message and enum types in `ServerMasterSectionLevelUpPayload`.
pub mod server_master_section_level_up_payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServerMasterSectionLevelUpBody {
        #[prost(message, optional, tag = "1")]
        pub a_tar: ::core::option::Option<
            server_master_section_level_up_body::ServerMasterSectionLevelUpATar,
        >,
        #[prost(message, repeated, tag = "2")]
        pub a_rst: ::prost::alloc::vec::Vec<super::ARst>,
    }
    /// Nested message and enum types in `ServerMasterSectionLevelUpBody`.
    pub mod server_master_section_level_up_body {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ServerMasterSectionLevelUpATar {
            #[prost(int64, tag = "1")]
            pub master_section_id: i64,
            #[prost(int64, tag = "2")]
            pub level_before: i64,
            #[prost(int64, tag = "3")]
            pub level_after: i64,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientGameStartPayload {
    #[prost(int64, tag = "1")]
    pub event_id: i64,
    #[prost(int64, tag = "2")]
    pub action_id: i64,
    #[prost(string, tag = "3")]
    pub action_name: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientMoviePlay {
    #[prost(int64, tag = "1")]
    pub event_id: i64,
    #[prost(int64, tag = "2")]
    pub movie_id: i64,
    #[prost(string, tag = "3")]
    pub movie_name: ::prost::alloc::string::String,
    #[prost(int64, tag = "4")]
    pub state: i64,
    #[prost(int64, tag = "5")]
    pub duration_time: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientLandIntoPayload {
    #[prost(int64, tag = "1")]
    pub event_id: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientSdkRegPayload {
    #[prost(int64, tag = "1")]
    pub event_id: i64,
    #[prost(int64, tag = "2")]
    pub state: i64,
    #[prost(int64, tag = "3")]
    pub code_id: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientSdkLoginPayload {
    #[prost(int64, tag = "1")]
    pub event_id: i64,
    #[prost(int64, tag = "2")]
    pub state: i64,
    #[prost(string, tag = "3")]
    pub code_id: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientUpdateAfterSdkPayload {
    #[prost(int64, tag = "1")]
    pub event_id: i64,
    #[prost(int64, tag = "2")]
    pub action_id: i64,
    #[prost(string, tag = "3")]
    pub action_name: ::prost::alloc::string::String,
    #[prost(int64, tag = "4")]
    pub pack: i64,
    #[prost(int64, tag = "5")]
    pub state: i64,
    #[prost(int64, tag = "6")]
    pub duration_time: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientGameLoadingPayload {
    #[prost(int64, tag = "1")]
    pub event_id: i64,
    #[prost(int64, tag = "2")]
    pub action_id: i64,
    #[prost(string, tag = "3")]
    pub action_name: ::prost::alloc::string::String,
    #[prost(int64, tag = "4")]
    pub state: i64,
    #[prost(int64, tag = "5")]
    pub duration_time: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientRoleCreatePayload {
    #[prost(int64, tag = "1")]
    pub event_id: i64,
    #[prost(int64, tag = "2")]
    pub state: i64,
    #[prost(string, tag = "3")]
    pub fail_reason: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientRoleLoginPayload {
    #[prost(int64, tag = "1")]
    pub event_id: i64,
    #[prost(int64, tag = "2")]
    pub state: i64,
    #[prost(string, tag = "3")]
    pub fail_reason: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientHallIntoPayload {
    #[prost(int64, tag = "1")]
    pub event_id: i64,
    #[prost(int64, tag = "2")]
    pub duration_time: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientRoleLogoutPayload {
    #[prost(int64, tag = "1")]
    pub event_id: i64,
    #[prost(int64, tag = "2")]
    pub scene_id: i64,
    #[prost(int64, tag = "3")]
    pub state: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientPageArrivePayload {
    #[prost(int64, tag = "1")]
    pub event_id: i64,
    #[prost(int64, tag = "2")]
    pub level_page_id: i64,
    #[prost(int64, tag = "3")]
    pub arrive_page_id: i64,
    #[prost(int64, tag = "4")]
    pub state: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientButtonClickPayload {
    #[prost(int64, tag = "1")]
    pub event_id: i64,
    #[prost(string, tag = "2")]
    pub button_type: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    pub button_id: i64,
    #[prost(string, tag = "4")]
    pub button_name: ::prost::alloc::string::String,
    #[prost(int64, tag = "5")]
    pub state: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ServerEventId {
    ServerInvalidEvent = 0,
    ServerRoleSnapshot = 100,
    ServerRoleCreate = 101,
    ServerRoleLogin = 102,
    ServerRoleGuide = 104,
    ServerRoleLevelup = 105,
    ServerRoleRename = 106,
    ServerRefreshTargetList = 107,
    ServerUnlockTargets = 108,
    ServerFinishTargets = 109,
    ServerFunctionUnlock = 110,
    ServerMonthlyCheckin = 111,
    ServerGiftCodeRedeem = 112,
    ServerRoleDelete = 113,
    ServerGameShare = 114,
    ServerRoleSetAvatar = 115,
    ServerRoleFrameChange = 116,
    ServerStageEnter = 200,
    ServerStageFight = 202,
    ServerStagePlot = 203,
    ServerStageFinish = 204,
    ServerStageSweep = 205,
    ServerBattleStars = 207,
    ServerStageFightStart = 211,
    ServerStageFightEnd = 212,
    ServerBattleCheat = 299,
    ServerExploreUnlock = 300,
    ServerExploreStart = 301,
    ServerExploreQuit = 302,
    ServerExploreTalk = 303,
    ServerExploreFight = 304,
    ServerExploreTask = 305,
    ServerUseKeyword = 306,
    ServerExploreFinish = 307,
    ServerExplorePuzzle = 308,
    ServerExplorePuzzleDrinkMaking = 309,
    ServerExploreFree = 310,
    ServerPresentSend = 311,
    ServerExploreGachaMachine = 312,
    ServerAutoDrinkMaking = 313,
    ServerMusicFight = 400,
    ServerMusicSweep = 401,
    ServerDqFight = 500,
    ServerDqSweep = 501,
    ServerDqSweepStart = 502,
    ServerDqSweepEnd = 503,
    ServerDqFightStart = 504,
    ServerDqFightEnd = 505,
    ServerBzFight = 601,
    ServerBzChests = 602,
    ServerBzStage = 603,
    ServerHometownShow = 700,
    ServerHometownDaily = 701,
    ServerCardSnapshot = 800,
    ServerCardLevelup = 802,
    ServerCardStarup = 803,
    ServerCardGradeup = 805,
    ServerCardFeeling = 807,
    ServerCardBondReward = 808,
    ServerContractUnlock = 809,
    ServerCardResetRank = 813,
    ServerEquipSnapshot = 900,
    ServerEquipLevelup = 901,
    ServerEquipStarup = 902,
    ServerEquipGradeup = 903,
    ServerEquipArrange = 904,
    ServerEquipWakeup = 905,
    ServerEquipDecompose = 906,
    ServerPhonemeSnapshot = 910,
    ServerPhonemeLevelup = 911,
    ServerPhonemeArrange = 912,
    ServerPhonemeDecompose = 913,
    ServerPhonemeBuffGacha = 914,
    ServerPhonemeBuffConfirm = 915,
    ServerPhonemeSuitArrange = 916,
    ServerTaskReach = 1001,
    ServerTaskReward = 1002,
    ServerTaskAllReward = 1003,
    ServerTaskPointReward = 1004,
    ServerActivityCheckinReward = 1005,
    ServerActivityMissionReward = 1006,
    ServerActivityMissionAllReward = 1007,
    ServerNewbieReceiveLevelRewards = 1008,
    ServerBadgeReceive = 1011,
    ServerEmailReceive = 1101,
    ServerEmailOpen = 1102,
    ServerEmailReward = 1103,
    ServerQuestionnaireSubmit = 1110,
    ServerItemSnapshot = 1200,
    ServerItemUse = 1201,
    ServerShopBuy = 1203,
    ServerCoinBuy = 1205,
    ServerMooncardBuy = 1208,
    ServerMooncardReward = 1209,
    ServerGameBillCreate = 1210,
    ServerGameBillClaim = 1211,
    ServerGameBillFinish = 1212,
    ServerGameBillCancel = 1213,
    ServerBpBuy = 1214,
    ServerBpLevelBuy = 1215,
    ServerBpRewards = 1216,
    ServerBpAllRewards = 1217,
    ServerGemToStone = 1218,
    ServerFirstPurchaseFinish = 1219,
    ServerGameGacha = 1300,
    ServerGameGachaPrefer = 1301,
    ServerUndergroundFinish = 1400,
    ServerUndergroundClaim = 1401,
    ServerUndergroundSetCaptain = 1402,
    ServerUndergroundStartSection = 1403,
    ServerUndergroundStartChapter = 1404,
    ServerUndergroundUnlockSection = 1405,
    ServerUndergroundUnlockChapter = 1406,
    ServerUndergroundStartBattle = 1407,
    ServerUndergroundFinishBattle = 1408,
    ServerCrusadeChoose = 1500,
    ServerCrusadeSetDeck = 1501,
    ServerCrusadeMove = 1502,
    ServerCrusadeEventStart = 1503,
    ServerCrusadeEventFinish = 1504,
    ServerCrusadeRestart = 1505,
    ServerCrusadeFinish = 1506,
    ServerCrusadeBattleStart = 1507,
    ServerCrusadeBattleFinish = 1508,
    ServerCrusadeActivityEnter = 1510,
    ServerCrusadeActivityArriveNode = 1511,
    ServerCrusadeCollectionRewards = 1512,
    ServerCrusadeActivityEventStart = 1513,
    ServerCrusadeActivityEventFinish = 1514,
    ServerCrusadeUndergroundStart = 1515,
    ServerCrusadeUndergroundFinish = 1516,
    ServerBattleActivityFight = 1600,
    ServerBattleActivityPlot = 1601,
    ServerBattleActivitySweepStart = 1602,
    ServerBattleActivitySweepEnd = 1603,
    ServerBattleActivityFightStart = 1604,
    ServerBattleActivityFightEnd = 1605,
    ServerOutfitPurchase = 1701,
    ServerOutfitUse = 1702,
    ServerMasterSectionBattleStart = 1801,
    ServerMasterSectionBattleEnd = 1802,
    ServerMasterSectionLevelUp = 1803,
    ServerDeviceBind = 20001,
}
impl ServerEventId {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ServerEventId::ServerInvalidEvent => "server_invalid_event",
            ServerEventId::ServerRoleSnapshot => "server_role_snapshot",
            ServerEventId::ServerRoleCreate => "server_role_create",
            ServerEventId::ServerRoleLogin => "server_role_login",
            ServerEventId::ServerRoleGuide => "server_role_guide",
            ServerEventId::ServerRoleLevelup => "server_role_levelup",
            ServerEventId::ServerRoleRename => "server_role_rename",
            ServerEventId::ServerRefreshTargetList => "server_refresh_target_list",
            ServerEventId::ServerUnlockTargets => "server_unlock_targets",
            ServerEventId::ServerFinishTargets => "server_finish_targets",
            ServerEventId::ServerFunctionUnlock => "server_function_unlock",
            ServerEventId::ServerMonthlyCheckin => "server_monthly_checkin",
            ServerEventId::ServerGiftCodeRedeem => "server_gift_code_redeem",
            ServerEventId::ServerRoleDelete => "server_role_delete",
            ServerEventId::ServerGameShare => "server_game_share",
            ServerEventId::ServerRoleSetAvatar => "server_role_set_avatar",
            ServerEventId::ServerRoleFrameChange => "server_role_frame_change",
            ServerEventId::ServerStageEnter => "server_stage_enter",
            ServerEventId::ServerStageFight => "server_stage_fight",
            ServerEventId::ServerStagePlot => "server_stage_plot",
            ServerEventId::ServerStageFinish => "server_stage_finish",
            ServerEventId::ServerStageSweep => "server_stage_sweep",
            ServerEventId::ServerBattleStars => "server_battle_stars",
            ServerEventId::ServerStageFightStart => "server_stage_fight_start",
            ServerEventId::ServerStageFightEnd => "server_stage_fight_end",
            ServerEventId::ServerBattleCheat => "server_battle_cheat",
            ServerEventId::ServerExploreUnlock => "server_explore_unlock",
            ServerEventId::ServerExploreStart => "server_explore_start",
            ServerEventId::ServerExploreQuit => "server_explore_quit",
            ServerEventId::ServerExploreTalk => "server_explore_talk",
            ServerEventId::ServerExploreFight => "server_explore_fight",
            ServerEventId::ServerExploreTask => "server_explore_task",
            ServerEventId::ServerUseKeyword => "server_use_keyword",
            ServerEventId::ServerExploreFinish => "server_explore_finish",
            ServerEventId::ServerExplorePuzzle => "server_explore_puzzle",
            ServerEventId::ServerExplorePuzzleDrinkMaking => {
                "server_explore_puzzle_drink_making"
            }
            ServerEventId::ServerExploreFree => "server_explore_free",
            ServerEventId::ServerPresentSend => "server_present_send",
            ServerEventId::ServerExploreGachaMachine => "server_explore_gacha_machine",
            ServerEventId::ServerAutoDrinkMaking => "server_auto_drink_making",
            ServerEventId::ServerMusicFight => "server_music_fight",
            ServerEventId::ServerMusicSweep => "server_music_sweep",
            ServerEventId::ServerDqFight => "server_dq_fight",
            ServerEventId::ServerDqSweep => "server_dq_sweep",
            ServerEventId::ServerDqSweepStart => "server_dq_sweep_start",
            ServerEventId::ServerDqSweepEnd => "server_dq_sweep_end",
            ServerEventId::ServerDqFightStart => "server_dq_fight_start",
            ServerEventId::ServerDqFightEnd => "server_dq_fight_end",
            ServerEventId::ServerBzFight => "server_bz_fight",
            ServerEventId::ServerBzChests => "server_bz_chests",
            ServerEventId::ServerBzStage => "server_bz_stage",
            ServerEventId::ServerHometownShow => "server_hometown_show",
            ServerEventId::ServerHometownDaily => "server_hometown_daily",
            ServerEventId::ServerCardSnapshot => "server_card_snapshot",
            ServerEventId::ServerCardLevelup => "server_card_levelup",
            ServerEventId::ServerCardStarup => "server_card_starup",
            ServerEventId::ServerCardGradeup => "server_card_gradeup",
            ServerEventId::ServerCardFeeling => "server_card_feeling",
            ServerEventId::ServerCardBondReward => "server_card_bond_reward",
            ServerEventId::ServerContractUnlock => "server_contract_unlock",
            ServerEventId::ServerCardResetRank => "server_card_reset_rank",
            ServerEventId::ServerEquipSnapshot => "server_equip_snapshot",
            ServerEventId::ServerEquipLevelup => "server_equip_levelup",
            ServerEventId::ServerEquipStarup => "server_equip_starup",
            ServerEventId::ServerEquipGradeup => "server_equip_gradeup",
            ServerEventId::ServerEquipArrange => "server_equip_arrange",
            ServerEventId::ServerEquipWakeup => "server_equip_wakeup",
            ServerEventId::ServerEquipDecompose => "server_equip_decompose",
            ServerEventId::ServerPhonemeSnapshot => "server_phoneme_snapshot",
            ServerEventId::ServerPhonemeLevelup => "server_phoneme_levelup",
            ServerEventId::ServerPhonemeArrange => "server_phoneme_arrange",
            ServerEventId::ServerPhonemeDecompose => "server_phoneme_decompose",
            ServerEventId::ServerPhonemeBuffGacha => "server_phoneme_buff_gacha",
            ServerEventId::ServerPhonemeBuffConfirm => "server_phoneme_buff_confirm",
            ServerEventId::ServerPhonemeSuitArrange => "server_phoneme_suit_arrange",
            ServerEventId::ServerTaskReach => "server_task_reach",
            ServerEventId::ServerTaskReward => "server_task_reward",
            ServerEventId::ServerTaskAllReward => "server_task_all_reward",
            ServerEventId::ServerTaskPointReward => "server_task_point_reward",
            ServerEventId::ServerActivityCheckinReward => {
                "server_activity_checkin_reward"
            }
            ServerEventId::ServerActivityMissionReward => {
                "server_activity_mission_reward"
            }
            ServerEventId::ServerActivityMissionAllReward => {
                "server_activity_mission_all_reward"
            }
            ServerEventId::ServerNewbieReceiveLevelRewards => {
                "server_newbie_receive_level_rewards"
            }
            ServerEventId::ServerBadgeReceive => "server_badge_receive",
            ServerEventId::ServerEmailReceive => "server_email_receive",
            ServerEventId::ServerEmailOpen => "server_email_open",
            ServerEventId::ServerEmailReward => "server_email_reward",
            ServerEventId::ServerQuestionnaireSubmit => "server_questionnaire_submit",
            ServerEventId::ServerItemSnapshot => "server_item_snapshot",
            ServerEventId::ServerItemUse => "server_item_use",
            ServerEventId::ServerShopBuy => "server_shop_buy",
            ServerEventId::ServerCoinBuy => "server_coin_buy",
            ServerEventId::ServerMooncardBuy => "server_mooncard_buy",
            ServerEventId::ServerMooncardReward => "server_mooncard_reward",
            ServerEventId::ServerGameBillCreate => "server_game_bill_create",
            ServerEventId::ServerGameBillClaim => "server_game_bill_claim",
            ServerEventId::ServerGameBillFinish => "server_game_bill_finish",
            ServerEventId::ServerGameBillCancel => "server_game_bill_cancel",
            ServerEventId::ServerBpBuy => "server_bp_buy",
            ServerEventId::ServerBpLevelBuy => "server_bp_level_buy",
            ServerEventId::ServerBpRewards => "server_bp_rewards",
            ServerEventId::ServerBpAllRewards => "server_bp_all_rewards",
            ServerEventId::ServerGemToStone => "server_gem_to_stone",
            ServerEventId::ServerFirstPurchaseFinish => "server_first_purchase_finish",
            ServerEventId::ServerGameGacha => "server_game_gacha",
            ServerEventId::ServerGameGachaPrefer => "server_game_gacha_prefer",
            ServerEventId::ServerUndergroundFinish => "server_underground_finish",
            ServerEventId::ServerUndergroundClaim => "server_underground_claim",
            ServerEventId::ServerUndergroundSetCaptain => {
                "server_underground_set_captain"
            }
            ServerEventId::ServerUndergroundStartSection => {
                "server_underground_start_section"
            }
            ServerEventId::ServerUndergroundStartChapter => {
                "server_underground_start_chapter"
            }
            ServerEventId::ServerUndergroundUnlockSection => {
                "server_underground_unlock_section"
            }
            ServerEventId::ServerUndergroundUnlockChapter => {
                "server_underground_unlock_chapter"
            }
            ServerEventId::ServerUndergroundStartBattle => {
                "server_underground_start_battle"
            }
            ServerEventId::ServerUndergroundFinishBattle => {
                "server_underground_finish_battle"
            }
            ServerEventId::ServerCrusadeChoose => "server_crusade_choose",
            ServerEventId::ServerCrusadeSetDeck => "server_crusade_set_deck",
            ServerEventId::ServerCrusadeMove => "server_crusade_move",
            ServerEventId::ServerCrusadeEventStart => "server_crusade_event_start",
            ServerEventId::ServerCrusadeEventFinish => "server_crusade_event_finish",
            ServerEventId::ServerCrusadeRestart => "server_crusade_restart",
            ServerEventId::ServerCrusadeFinish => "server_crusade_finish",
            ServerEventId::ServerCrusadeBattleStart => "server_crusade_battle_start",
            ServerEventId::ServerCrusadeBattleFinish => "server_crusade_battle_finish",
            ServerEventId::ServerCrusadeActivityEnter => "server_crusade_activity_enter",
            ServerEventId::ServerCrusadeActivityArriveNode => {
                "server_crusade_activity_arrive_node"
            }
            ServerEventId::ServerCrusadeCollectionRewards => {
                "server_crusade_collection_rewards"
            }
            ServerEventId::ServerCrusadeActivityEventStart => {
                "server_crusade_activity_event_start"
            }
            ServerEventId::ServerCrusadeActivityEventFinish => {
                "server_crusade_activity_event_finish"
            }
            ServerEventId::ServerCrusadeUndergroundStart => {
                "server_crusade_underground_start"
            }
            ServerEventId::ServerCrusadeUndergroundFinish => {
                "server_crusade_underground_finish"
            }
            ServerEventId::ServerBattleActivityFight => "server_battle_activity_fight",
            ServerEventId::ServerBattleActivityPlot => "server_battle_activity_plot",
            ServerEventId::ServerBattleActivitySweepStart => {
                "server_battle_activity_sweep_start"
            }
            ServerEventId::ServerBattleActivitySweepEnd => {
                "server_battle_activity_sweep_end"
            }
            ServerEventId::ServerBattleActivityFightStart => {
                "server_battle_activity_fight_start"
            }
            ServerEventId::ServerBattleActivityFightEnd => {
                "server_battle_activity_fight_end"
            }
            ServerEventId::ServerOutfitPurchase => "server_outfit_purchase",
            ServerEventId::ServerOutfitUse => "server_outfit_use",
            ServerEventId::ServerMasterSectionBattleStart => {
                "server_master_section_battle_start"
            }
            ServerEventId::ServerMasterSectionBattleEnd => {
                "server_master_section_battle_end"
            }
            ServerEventId::ServerMasterSectionLevelUp => "server_master_section_level_up",
            ServerEventId::ServerDeviceBind => "server_device_bind",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "server_invalid_event" => Some(Self::ServerInvalidEvent),
            "server_role_snapshot" => Some(Self::ServerRoleSnapshot),
            "server_role_create" => Some(Self::ServerRoleCreate),
            "server_role_login" => Some(Self::ServerRoleLogin),
            "server_role_guide" => Some(Self::ServerRoleGuide),
            "server_role_levelup" => Some(Self::ServerRoleLevelup),
            "server_role_rename" => Some(Self::ServerRoleRename),
            "server_refresh_target_list" => Some(Self::ServerRefreshTargetList),
            "server_unlock_targets" => Some(Self::ServerUnlockTargets),
            "server_finish_targets" => Some(Self::ServerFinishTargets),
            "server_function_unlock" => Some(Self::ServerFunctionUnlock),
            "server_monthly_checkin" => Some(Self::ServerMonthlyCheckin),
            "server_gift_code_redeem" => Some(Self::ServerGiftCodeRedeem),
            "server_role_delete" => Some(Self::ServerRoleDelete),
            "server_game_share" => Some(Self::ServerGameShare),
            "server_role_set_avatar" => Some(Self::ServerRoleSetAvatar),
            "server_role_frame_change" => Some(Self::ServerRoleFrameChange),
            "server_stage_enter" => Some(Self::ServerStageEnter),
            "server_stage_fight" => Some(Self::ServerStageFight),
            "server_stage_plot" => Some(Self::ServerStagePlot),
            "server_stage_finish" => Some(Self::ServerStageFinish),
            "server_stage_sweep" => Some(Self::ServerStageSweep),
            "server_battle_stars" => Some(Self::ServerBattleStars),
            "server_stage_fight_start" => Some(Self::ServerStageFightStart),
            "server_stage_fight_end" => Some(Self::ServerStageFightEnd),
            "server_battle_cheat" => Some(Self::ServerBattleCheat),
            "server_explore_unlock" => Some(Self::ServerExploreUnlock),
            "server_explore_start" => Some(Self::ServerExploreStart),
            "server_explore_quit" => Some(Self::ServerExploreQuit),
            "server_explore_talk" => Some(Self::ServerExploreTalk),
            "server_explore_fight" => Some(Self::ServerExploreFight),
            "server_explore_task" => Some(Self::ServerExploreTask),
            "server_use_keyword" => Some(Self::ServerUseKeyword),
            "server_explore_finish" => Some(Self::ServerExploreFinish),
            "server_explore_puzzle" => Some(Self::ServerExplorePuzzle),
            "server_explore_puzzle_drink_making" => {
                Some(Self::ServerExplorePuzzleDrinkMaking)
            }
            "server_explore_free" => Some(Self::ServerExploreFree),
            "server_present_send" => Some(Self::ServerPresentSend),
            "server_explore_gacha_machine" => Some(Self::ServerExploreGachaMachine),
            "server_auto_drink_making" => Some(Self::ServerAutoDrinkMaking),
            "server_music_fight" => Some(Self::ServerMusicFight),
            "server_music_sweep" => Some(Self::ServerMusicSweep),
            "server_dq_fight" => Some(Self::ServerDqFight),
            "server_dq_sweep" => Some(Self::ServerDqSweep),
            "server_dq_sweep_start" => Some(Self::ServerDqSweepStart),
            "server_dq_sweep_end" => Some(Self::ServerDqSweepEnd),
            "server_dq_fight_start" => Some(Self::ServerDqFightStart),
            "server_dq_fight_end" => Some(Self::ServerDqFightEnd),
            "server_bz_fight" => Some(Self::ServerBzFight),
            "server_bz_chests" => Some(Self::ServerBzChests),
            "server_bz_stage" => Some(Self::ServerBzStage),
            "server_hometown_show" => Some(Self::ServerHometownShow),
            "server_hometown_daily" => Some(Self::ServerHometownDaily),
            "server_card_snapshot" => Some(Self::ServerCardSnapshot),
            "server_card_levelup" => Some(Self::ServerCardLevelup),
            "server_card_starup" => Some(Self::ServerCardStarup),
            "server_card_gradeup" => Some(Self::ServerCardGradeup),
            "server_card_feeling" => Some(Self::ServerCardFeeling),
            "server_card_bond_reward" => Some(Self::ServerCardBondReward),
            "server_contract_unlock" => Some(Self::ServerContractUnlock),
            "server_card_reset_rank" => Some(Self::ServerCardResetRank),
            "server_equip_snapshot" => Some(Self::ServerEquipSnapshot),
            "server_equip_levelup" => Some(Self::ServerEquipLevelup),
            "server_equip_starup" => Some(Self::ServerEquipStarup),
            "server_equip_gradeup" => Some(Self::ServerEquipGradeup),
            "server_equip_arrange" => Some(Self::ServerEquipArrange),
            "server_equip_wakeup" => Some(Self::ServerEquipWakeup),
            "server_equip_decompose" => Some(Self::ServerEquipDecompose),
            "server_phoneme_snapshot" => Some(Self::ServerPhonemeSnapshot),
            "server_phoneme_levelup" => Some(Self::ServerPhonemeLevelup),
            "server_phoneme_arrange" => Some(Self::ServerPhonemeArrange),
            "server_phoneme_decompose" => Some(Self::ServerPhonemeDecompose),
            "server_phoneme_buff_gacha" => Some(Self::ServerPhonemeBuffGacha),
            "server_phoneme_buff_confirm" => Some(Self::ServerPhonemeBuffConfirm),
            "server_phoneme_suit_arrange" => Some(Self::ServerPhonemeSuitArrange),
            "server_task_reach" => Some(Self::ServerTaskReach),
            "server_task_reward" => Some(Self::ServerTaskReward),
            "server_task_all_reward" => Some(Self::ServerTaskAllReward),
            "server_task_point_reward" => Some(Self::ServerTaskPointReward),
            "server_activity_checkin_reward" => Some(Self::ServerActivityCheckinReward),
            "server_activity_mission_reward" => Some(Self::ServerActivityMissionReward),
            "server_activity_mission_all_reward" => {
                Some(Self::ServerActivityMissionAllReward)
            }
            "server_newbie_receive_level_rewards" => {
                Some(Self::ServerNewbieReceiveLevelRewards)
            }
            "server_badge_receive" => Some(Self::ServerBadgeReceive),
            "server_email_receive" => Some(Self::ServerEmailReceive),
            "server_email_open" => Some(Self::ServerEmailOpen),
            "server_email_reward" => Some(Self::ServerEmailReward),
            "server_questionnaire_submit" => Some(Self::ServerQuestionnaireSubmit),
            "server_item_snapshot" => Some(Self::ServerItemSnapshot),
            "server_item_use" => Some(Self::ServerItemUse),
            "server_shop_buy" => Some(Self::ServerShopBuy),
            "server_coin_buy" => Some(Self::ServerCoinBuy),
            "server_mooncard_buy" => Some(Self::ServerMooncardBuy),
            "server_mooncard_reward" => Some(Self::ServerMooncardReward),
            "server_game_bill_create" => Some(Self::ServerGameBillCreate),
            "server_game_bill_claim" => Some(Self::ServerGameBillClaim),
            "server_game_bill_finish" => Some(Self::ServerGameBillFinish),
            "server_game_bill_cancel" => Some(Self::ServerGameBillCancel),
            "server_bp_buy" => Some(Self::ServerBpBuy),
            "server_bp_level_buy" => Some(Self::ServerBpLevelBuy),
            "server_bp_rewards" => Some(Self::ServerBpRewards),
            "server_bp_all_rewards" => Some(Self::ServerBpAllRewards),
            "server_gem_to_stone" => Some(Self::ServerGemToStone),
            "server_first_purchase_finish" => Some(Self::ServerFirstPurchaseFinish),
            "server_game_gacha" => Some(Self::ServerGameGacha),
            "server_game_gacha_prefer" => Some(Self::ServerGameGachaPrefer),
            "server_underground_finish" => Some(Self::ServerUndergroundFinish),
            "server_underground_claim" => Some(Self::ServerUndergroundClaim),
            "server_underground_set_captain" => Some(Self::ServerUndergroundSetCaptain),
            "server_underground_start_section" => {
                Some(Self::ServerUndergroundStartSection)
            }
            "server_underground_start_chapter" => {
                Some(Self::ServerUndergroundStartChapter)
            }
            "server_underground_unlock_section" => {
                Some(Self::ServerUndergroundUnlockSection)
            }
            "server_underground_unlock_chapter" => {
                Some(Self::ServerUndergroundUnlockChapter)
            }
            "server_underground_start_battle" => Some(Self::ServerUndergroundStartBattle),
            "server_underground_finish_battle" => {
                Some(Self::ServerUndergroundFinishBattle)
            }
            "server_crusade_choose" => Some(Self::ServerCrusadeChoose),
            "server_crusade_set_deck" => Some(Self::ServerCrusadeSetDeck),
            "server_crusade_move" => Some(Self::ServerCrusadeMove),
            "server_crusade_event_start" => Some(Self::ServerCrusadeEventStart),
            "server_crusade_event_finish" => Some(Self::ServerCrusadeEventFinish),
            "server_crusade_restart" => Some(Self::ServerCrusadeRestart),
            "server_crusade_finish" => Some(Self::ServerCrusadeFinish),
            "server_crusade_battle_start" => Some(Self::ServerCrusadeBattleStart),
            "server_crusade_battle_finish" => Some(Self::ServerCrusadeBattleFinish),
            "server_crusade_activity_enter" => Some(Self::ServerCrusadeActivityEnter),
            "server_crusade_activity_arrive_node" => {
                Some(Self::ServerCrusadeActivityArriveNode)
            }
            "server_crusade_collection_rewards" => {
                Some(Self::ServerCrusadeCollectionRewards)
            }
            "server_crusade_activity_event_start" => {
                Some(Self::ServerCrusadeActivityEventStart)
            }
            "server_crusade_activity_event_finish" => {
                Some(Self::ServerCrusadeActivityEventFinish)
            }
            "server_crusade_underground_start" => {
                Some(Self::ServerCrusadeUndergroundStart)
            }
            "server_crusade_underground_finish" => {
                Some(Self::ServerCrusadeUndergroundFinish)
            }
            "server_battle_activity_fight" => Some(Self::ServerBattleActivityFight),
            "server_battle_activity_plot" => Some(Self::ServerBattleActivityPlot),
            "server_battle_activity_sweep_start" => {
                Some(Self::ServerBattleActivitySweepStart)
            }
            "server_battle_activity_sweep_end" => {
                Some(Self::ServerBattleActivitySweepEnd)
            }
            "server_battle_activity_fight_start" => {
                Some(Self::ServerBattleActivityFightStart)
            }
            "server_battle_activity_fight_end" => {
                Some(Self::ServerBattleActivityFightEnd)
            }
            "server_outfit_purchase" => Some(Self::ServerOutfitPurchase),
            "server_outfit_use" => Some(Self::ServerOutfitUse),
            "server_master_section_battle_start" => {
                Some(Self::ServerMasterSectionBattleStart)
            }
            "server_master_section_battle_end" => {
                Some(Self::ServerMasterSectionBattleEnd)
            }
            "server_master_section_level_up" => Some(Self::ServerMasterSectionLevelUp),
            "server_device_bind" => Some(Self::ServerDeviceBind),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ClientEventId {
    InvalidClientEventId = 0,
    ClientGameStart = 100010,
    ClientMoviePlay = 10020,
    ClientLandInto = 100030,
    ClientSdkReg = 100040,
    ClientSdkLogin = 100050,
    ClientUpdateAftersdk = 100060,
    ClientGameLoading = 100090,
    ClientRoleCreate = 100100,
    ClientRoleLogin = 100110,
    ClientRoleGuide = 100120,
    ClientHallInto = 100130,
    ClientRoleLogout = 100140,
}
impl ClientEventId {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ClientEventId::InvalidClientEventId => "invalid_client_event_id",
            ClientEventId::ClientGameStart => "client_game_start",
            ClientEventId::ClientMoviePlay => "client_movie_play",
            ClientEventId::ClientLandInto => "client_land_into",
            ClientEventId::ClientSdkReg => "client_sdk_reg",
            ClientEventId::ClientSdkLogin => "client_sdk_login",
            ClientEventId::ClientUpdateAftersdk => "client_update_aftersdk",
            ClientEventId::ClientGameLoading => "client_game_loading",
            ClientEventId::ClientRoleCreate => "client_role_create",
            ClientEventId::ClientRoleLogin => "client_role_login",
            ClientEventId::ClientRoleGuide => "client_role_guide",
            ClientEventId::ClientHallInto => "client_hall_into",
            ClientEventId::ClientRoleLogout => "client_role_logout",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "invalid_client_event_id" => Some(Self::InvalidClientEventId),
            "client_game_start" => Some(Self::ClientGameStart),
            "client_movie_play" => Some(Self::ClientMoviePlay),
            "client_land_into" => Some(Self::ClientLandInto),
            "client_sdk_reg" => Some(Self::ClientSdkReg),
            "client_sdk_login" => Some(Self::ClientSdkLogin),
            "client_update_aftersdk" => Some(Self::ClientUpdateAftersdk),
            "client_game_loading" => Some(Self::ClientGameLoading),
            "client_role_create" => Some(Self::ClientRoleCreate),
            "client_role_login" => Some(Self::ClientRoleLogin),
            "client_role_guide" => Some(Self::ClientRoleGuide),
            "client_hall_into" => Some(Self::ClientHallInto),
            "client_role_logout" => Some(Self::ClientRoleLogout),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TaskType {
    InvalidTaskType = 0,
    Daily = 1,
    Achievement = 2,
    Activity = 3,
}
impl TaskType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TaskType::InvalidTaskType => "InvalidTaskType",
            TaskType::Daily => "Daily",
            TaskType::Achievement => "Achievement",
            TaskType::Activity => "Activity",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "InvalidTaskType" => Some(Self::InvalidTaskType),
            "Daily" => Some(Self::Daily),
            "Achievement" => Some(Self::Achievement),
            "Activity" => Some(Self::Activity),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SweepType {
    InvalidSweepType = 0,
    SingleSweep = 1,
    TenSweeps = 2,
}
impl SweepType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SweepType::InvalidSweepType => "InvalidSweepType",
            SweepType::SingleSweep => "SingleSweep",
            SweepType::TenSweeps => "TenSweeps",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "InvalidSweepType" => Some(Self::InvalidSweepType),
            "SingleSweep" => Some(Self::SingleSweep),
            "TenSweeps" => Some(Self::TenSweeps),
            _ => None,
        }
    }
}
