#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerPreference {
    #[prost(sint64, tag = "2")]
    pub birth_year: i64,
    #[prost(sint64, tag = "3")]
    pub birth_month: i64,
    #[prost(string, tag = "4")]
    pub country: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub language: ::prost::alloc::string::String,
    #[prost(bool, tag = "6")]
    pub opt_out: bool,
    #[prost(string, tag = "7")]
    pub consented_tos_version: ::prost::alloc::string::String,
    #[prost(sint64, tag = "8")]
    pub created_at: i64,
    #[prost(sint64, tag = "9")]
    pub updated_at: i64,
    #[prost(string, tag = "10")]
    pub nickname: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub consented_privacy_policy_version: ::prost::alloc::string::String,
    #[prost(sint64, tag = "12")]
    pub nickname_updated_at: i64,
    #[prost(enumeration = "AllowFriendRequestType", tag = "13")]
    pub allow_friend_request: i32,
    #[prost(sint64, tag = "14")]
    pub max_friend_number: i64,
    #[prost(sint64, tag = "15")]
    pub max_friend_request_queue_length: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AllowFriendRequestType {
    NoPreference = 0,
    AllowAll = 1,
    ForbidAll = 2,
}
impl AllowFriendRequestType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AllowFriendRequestType::NoPreference => "NO_PREFERENCE",
            AllowFriendRequestType::AllowAll => "ALLOW_ALL",
            AllowFriendRequestType::ForbidAll => "FORBID_ALL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NO_PREFERENCE" => Some(Self::NoPreference),
            "ALLOW_ALL" => Some(Self::AllowAll),
            "FORBID_ALL" => Some(Self::ForbidAll),
            _ => None,
        }
    }
}
