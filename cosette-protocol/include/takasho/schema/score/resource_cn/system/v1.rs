#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetsInfo {
    #[prost(string, tag = "1")]
    pub master_data_url: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub assets_bundle_url: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub master_data_version: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub assets_bundle_version: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub domain: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommonResponse {
    #[prost(message, optional, tag = "1")]
    pub player: ::core::option::Option<super::super::player::v1::Player>,
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<super::super::item::v1::PlayerItem>,
    #[prost(message, repeated, tag = "3")]
    pub battle_characters: ::prost::alloc::vec::Vec<
        super::super::character::v1::PlayerBattleCharacter,
    >,
    #[prost(message, repeated, tag = "4")]
    pub equipments: ::prost::alloc::vec::Vec<
        super::super::equipment::v1::PlayerEquipment,
    >,
    #[prost(message, optional, tag = "5")]
    pub wallet: ::core::option::Option<super::super::player::v1::PlayerWalletResponse>,
    #[prost(message, optional, tag = "6")]
    pub tutorial_info: ::core::option::Option<
        super::super::player::v1::PlayerTutorialInfo,
    >,
    #[prost(message, optional, tag = "7")]
    pub player_preference: ::core::option::Option<
        super::super::super::super::common_featureset::resource::player_preference::v1::PlayerPreference,
    >,
    #[prost(message, repeated, tag = "8")]
    pub bonds: ::prost::alloc::vec::Vec<
        super::super::character::v1::PlayerBattleCharacterBond,
    >,
    #[prost(int64, tag = "9")]
    pub received_energy: i64,
    #[prost(int64, tag = "10")]
    pub delta_player_level: i64,
    #[prost(message, repeated, tag = "21")]
    pub chapters: ::prost::alloc::vec::Vec<super::super::chapter::v1::PlayerChapter>,
    #[prost(int64, repeated, tag = "22")]
    pub opened_chapters: ::prost::alloc::vec::Vec<i64>,
    #[prost(message, repeated, tag = "23")]
    pub battle_zone: ::prost::alloc::vec::Vec<
        super::super::battle_zone::v1::PlayerBattleZone,
    >,
    #[prost(message, repeated, tag = "24")]
    pub decks: ::prost::alloc::vec::Vec<super::super::deck::v1::PlayerDeck>,
    #[prost(message, repeated, tag = "25")]
    pub keywords: ::prost::alloc::vec::Vec<super::super::keyword::v1::PlayerKeyword>,
    #[prost(int64, repeated, tag = "26")]
    pub missions: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, repeated, tag = "27")]
    pub unlocked_underground_chapters: ::prost::alloc::vec::Vec<i64>,
    #[prost(message, repeated, tag = "28")]
    pub badges: ::prost::alloc::vec::Vec<super::super::badge::v1::PlayerBadge>,
    #[prost(message, repeated, tag = "29")]
    pub functions: ::prost::alloc::vec::Vec<
        super::super::function_unlock::v1::PlayerFunctionUnlock,
    >,
    #[prost(message, repeated, tag = "30")]
    pub level_rewards: ::prost::alloc::vec::Vec<
        super::super::level_reward::v1::PlayerLevelReward,
    >,
    #[prost(message, repeated, tag = "31")]
    pub targets: ::prost::alloc::vec::Vec<super::super::target::v1::PlayerTarget>,
    #[prost(message, repeated, tag = "32")]
    pub phonemes: ::prost::alloc::vec::Vec<super::super::phoneme::v1::PlayerPhoneme>,
    #[prost(message, optional, tag = "33")]
    pub battle_pass_info: ::core::option::Option<
        super::super::battle_pass::v1::PlayerBattlePass,
    >,
    #[prost(message, optional, tag = "34")]
    pub monthly_pass_info: ::core::option::Option<
        super::super::monthly_pass::v1::PlayerMonthlyPass,
    >,
    #[prost(message, repeated, tag = "35")]
    pub pop_gifts: ::prost::alloc::vec::Vec<super::super::pop_gift::v1::PlayerPopGift>,
    #[prost(message, optional, tag = "36")]
    pub credit: ::core::option::Option<
        super::super::credit::v1::PlayerCreditPointResponse,
    >,
    #[prost(int64, repeated, tag = "37")]
    pub perfect_drink_making_types: ::prost::alloc::vec::Vec<i64>,
    #[prost(message, repeated, tag = "38")]
    pub disguises: ::prost::alloc::vec::Vec<super::super::disguise::v1::PlayerDisguise>,
    #[prost(message, repeated, tag = "39")]
    pub backgrounds: ::prost::alloc::vec::Vec<
        super::super::background::v1::PlayerBackground,
    >,
    #[prost(message, repeated, tag = "40")]
    pub icon_frames: ::prost::alloc::vec::Vec<
        super::super::icon_frame::v1::PlayerIconFrame,
    >,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SystemInfo {
    #[prost(message, optional, tag = "1")]
    pub maintenance: ::core::option::Option<
        super::super::global_server_settings::v1::Maintenance,
    >,
    #[prost(sint64, tag = "2")]
    pub server_time: i64,
    #[prost(string, tag = "3")]
    pub in_game_announcement_url: ::prost::alloc::string::String,
    #[prost(int64, tag = "4")]
    pub server_timezone: i64,
    #[prost(bool, tag = "5")]
    pub acquire_token: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Region {
    Invalid = 0,
    Cn = 1,
    Jp = 2,
    Ww = 3,
}
impl Region {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Region::Invalid => "INVALID",
            Region::Cn => "CN",
            Region::Jp => "JP",
            Region::Ww => "WW",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INVALID" => Some(Self::Invalid),
            "CN" => Some(Self::Cn),
            "JP" => Some(Self::Jp),
            "WW" => Some(Self::Ww),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetBundleVersion {
    #[prost(string, tag = "1")]
    pub asset_bundle_version_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub asset_bundle_version: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    #[prost(bool, tag = "4")]
    pub enable: bool,
    #[prost(
        enumeration = "super::super::super::super::common_featureset::resource::system::v1::PlatformType",
        tag = "5"
    )]
    pub platform: i32,
    #[prost(sint64, tag = "91")]
    pub created_at: i64,
    #[prost(sint64, tag = "92")]
    pub updated_at: i64,
}
