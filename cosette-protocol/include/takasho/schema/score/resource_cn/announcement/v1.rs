#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Announcement {
    #[prost(string, tag = "1")]
    pub announcement_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub affcodes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, tag = "3")]
    pub priority: i64,
    #[prost(string, tag = "4")]
    pub overview_bar_text: ::prost::alloc::string::String,
    #[prost(enumeration = "announcement::TagType", tag = "5")]
    pub overview_bar_tag: i32,
    #[prost(string, tag = "6")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub text: ::prost::alloc::string::String,
    #[prost(bool, tag = "8")]
    pub flag: bool,
    #[prost(sint64, tag = "9")]
    pub published_at: i64,
    #[prost(sint64, tag = "10")]
    pub end_at: i64,
    #[prost(string, tag = "11")]
    pub ja_title: ::prost::alloc::string::String,
    #[prost(string, tag = "12")]
    pub ja_content: ::prost::alloc::string::String,
    #[prost(string, tag = "13")]
    pub zh_tw_title: ::prost::alloc::string::String,
    #[prost(string, tag = "14")]
    pub zh_tw_content: ::prost::alloc::string::String,
    #[prost(string, tag = "15")]
    pub en_title: ::prost::alloc::string::String,
    #[prost(string, tag = "16")]
    pub en_content: ::prost::alloc::string::String,
    #[prost(string, tag = "17")]
    pub ko_title: ::prost::alloc::string::String,
    #[prost(string, tag = "18")]
    pub ko_content: ::prost::alloc::string::String,
    #[prost(string, tag = "19")]
    pub th_title: ::prost::alloc::string::String,
    #[prost(string, tag = "20")]
    pub th_content: ::prost::alloc::string::String,
    #[prost(string, tag = "21")]
    pub overview_bar_ja_text: ::prost::alloc::string::String,
    #[prost(string, tag = "22")]
    pub overview_bar_zh_tw_text: ::prost::alloc::string::String,
    #[prost(string, tag = "23")]
    pub overview_bar_en_text: ::prost::alloc::string::String,
    #[prost(string, tag = "24")]
    pub overview_bar_ko_text: ::prost::alloc::string::String,
    #[prost(string, tag = "25")]
    pub overview_bar_th_text: ::prost::alloc::string::String,
    #[prost(sint64, tag = "91")]
    pub created_at: i64,
    #[prost(sint64, tag = "92")]
    pub updated_at: i64,
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
    pub enum TagType {
        NoTag = 0,
        News = 1,
        Announcement = 2,
        Activity = 3,
    }
    impl TagType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                TagType::NoTag => "NO_TAG",
                TagType::News => "NEWS",
                TagType::Announcement => "ANNOUNCEMENT",
                TagType::Activity => "ACTIVITY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "NO_TAG" => Some(Self::NoTag),
                "NEWS" => Some(Self::News),
                "ANNOUNCEMENT" => Some(Self::Announcement),
                "ACTIVITY" => Some(Self::Activity),
                _ => None,
            }
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Announcements {
    #[prost(message, repeated, tag = "1")]
    pub announcements: ::prost::alloc::vec::Vec<Announcement>,
    #[prost(
        enumeration = "super::super::global_server_settings::v1::AnnouncementPopType",
        tag = "2"
    )]
    pub pop_type: i32,
}
