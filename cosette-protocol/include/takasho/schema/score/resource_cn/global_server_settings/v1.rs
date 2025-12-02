#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GlobalServerSettings {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub use_yidun_heartbeat_check: bool,
    #[prost(string, tag = "3")]
    pub db_maintenance: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub maintenance: ::core::option::Option<Maintenance>,
    #[prost(string, tag = "5")]
    pub in_game_announcement_url: ::prost::alloc::string::String,
    #[prost(bool, tag = "6")]
    pub open_battle_check: bool,
    #[prost(int64, tag = "8")]
    pub battle_min_dur_seconds: i64,
    #[prost(int64, tag = "9")]
    pub battle_min_dur_rounds: i64,
    #[prost(string, tag = "10")]
    pub server_name: ::prost::alloc::string::String,
    #[prost(enumeration = "AnnouncementPopType", tag = "11")]
    pub announcement_pop_type: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Maintenance {
    #[prost(bool, tag = "1")]
    pub is_active: bool,
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    #[prost(sint64, tag = "3")]
    pub start_at: i64,
    #[prost(sint64, tag = "4")]
    pub visible_at: i64,
    #[prost(string, tag = "5")]
    pub ja_message: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub zh_tw_message: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub en_message: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub ko_message: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub th_message: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AnnouncementPopType {
    InvalidType = 0,
    Daily = 1,
    NewAnnouncement = 2,
    NoPop = 3,
}
impl AnnouncementPopType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AnnouncementPopType::InvalidType => "INVALID_TYPE",
            AnnouncementPopType::Daily => "DAILY",
            AnnouncementPopType::NewAnnouncement => "NEW_ANNOUNCEMENT",
            AnnouncementPopType::NoPop => "NO_POP",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INVALID_TYPE" => Some(Self::InvalidType),
            "DAILY" => Some(Self::Daily),
            "NEW_ANNOUNCEMENT" => Some(Self::NewAnnouncement),
            "NO_POP" => Some(Self::NoPop),
            _ => None,
        }
    }
}
