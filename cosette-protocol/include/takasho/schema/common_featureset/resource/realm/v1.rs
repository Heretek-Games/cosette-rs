#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Realm {
    #[prost(int64, tag = "1")]
    pub initial_realm_id: i64,
    #[prost(int64, tag = "2")]
    pub current_realm_id: i64,
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    #[prost(enumeration = "Status", tag = "4")]
    pub status: i32,
    #[prost(int64, tag = "5")]
    pub open_at: i64,
    #[prost(string, tag = "6")]
    pub game_server: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub http_server: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub extra_info2: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub extra_info3: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Status {
    Invalid = 0,
    Hidden = 1,
    Visible = 2,
    Open = 3,
    Closed = 4,
}
impl Status {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Status::Invalid => "INVALID",
            Status::Hidden => "HIDDEN",
            Status::Visible => "VISIBLE",
            Status::Open => "OPEN",
            Status::Closed => "CLOSED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INVALID" => Some(Self::Invalid),
            "HIDDEN" => Some(Self::Hidden),
            "VISIBLE" => Some(Self::Visible),
            "OPEN" => Some(Self::Open),
            "CLOSED" => Some(Self::Closed),
            _ => None,
        }
    }
}
