#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoginActivityMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub login_activity_master_id: i64,
    #[prost(enumeration = "DisplayType", tag = "3")]
    pub display_type: i32,
    #[prost(enumeration = "LoginActivityType", tag = "4")]
    pub permanent_type: i32,
    #[prost(int64, tag = "5")]
    pub sort_id: i64,
    #[prost(string, tag = "6")]
    pub start_time: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub end_time: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoginActivityMasters {
    #[prost(message, repeated, tag = "1")]
    pub login_activity_masters: ::prost::alloc::vec::Vec<LoginActivityMaster>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DisplayType {
    None = 0,
    Video = 1,
    Signin = 2,
}
impl DisplayType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DisplayType::None => "NONE",
            DisplayType::Video => "VIDEO",
            DisplayType::Signin => "SIGNIN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NONE" => Some(Self::None),
            "VIDEO" => Some(Self::Video),
            "SIGNIN" => Some(Self::Signin),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LoginActivityType {
    Nopermanent = 0,
    Permanent = 1,
}
impl LoginActivityType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LoginActivityType::Nopermanent => "NOPERMANENT",
            LoginActivityType::Permanent => "PERMANENT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NOPERMANENT" => Some(Self::Nopermanent),
            "PERMANENT" => Some(Self::Permanent),
            _ => None,
        }
    }
}
