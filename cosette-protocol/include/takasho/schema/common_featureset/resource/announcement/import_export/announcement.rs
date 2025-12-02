#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Announcements {
    #[prost(message, repeated, tag = "1")]
    pub announcements: ::prost::alloc::vec::Vec<Announcement>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Announcement {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub language_code: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub title: ::prost::alloc::string::String,
    #[prost(sint64, tag = "4")]
    pub display_priority: i64,
    #[prost(sint64, tag = "5")]
    pub opened_at: i64,
    #[prost(sint64, tag = "6")]
    pub closed_at: i64,
    #[prost(string, tag = "7")]
    pub value_path: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "8")]
    pub contents: ::prost::alloc::vec::Vec<AnnouncementContent>,
    #[prost(enumeration = "announcement::PlatformType", tag = "9")]
    pub platform_type: i32,
}
/// Nested message and enum types in `Announcement`.
pub mod announcement {
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
    pub enum PlatformType {
        Unknown = 0,
        None = 1,
        Google = 2,
        Apple = 3,
        All = 99,
    }
    impl PlatformType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PlatformType::Unknown => "UNKNOWN",
                PlatformType::None => "NONE",
                PlatformType::Google => "GOOGLE",
                PlatformType::Apple => "APPLE",
                PlatformType::All => "ALL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNKNOWN" => Some(Self::Unknown),
                "NONE" => Some(Self::None),
                "GOOGLE" => Some(Self::Google),
                "APPLE" => Some(Self::Apple),
                "ALL" => Some(Self::All),
                _ => None,
            }
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnouncementContent {
    #[prost(string, tag = "1")]
    pub schema_key: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub value_path: ::prost::alloc::string::String,
}
