#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CrmGiftMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub crm_gift_master_id: i64,
    #[prost(int64, tag = "3")]
    pub crm_event_master_id: i64,
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub affcode: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub realm_id: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub start_time: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub end_time: ::prost::alloc::string::String,
    #[prost(enumeration = "LimitType", tag = "11")]
    pub limit_type: i32,
    #[prost(int64, tag = "12")]
    pub limit_count: i64,
    #[prost(int64, tag = "13")]
    pub mail_template_master_id: i64,
    #[prost(string, repeated, tag = "14")]
    pub rewards: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(sint64, tag = "91")]
    pub created_at: i64,
    #[prost(sint64, tag = "92")]
    pub updated_at: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CrmGiftMasters {
    #[prost(message, repeated, tag = "1")]
    pub crm_gift_masters: ::prost::alloc::vec::Vec<CrmGiftMaster>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LimitType {
    InvalidLimitType = 0,
    LimitIndividual = 1,
}
impl LimitType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LimitType::InvalidLimitType => "INVALID_LIMIT_TYPE",
            LimitType::LimitIndividual => "LIMIT_INDIVIDUAL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INVALID_LIMIT_TYPE" => Some(Self::InvalidLimitType),
            "LIMIT_INDIVIDUAL" => Some(Self::LimitIndividual),
            _ => None,
        }
    }
}
