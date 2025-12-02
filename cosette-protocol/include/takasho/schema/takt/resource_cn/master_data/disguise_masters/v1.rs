#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisguiseMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub disguise_master_id: i64,
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "5")]
    pub target: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(enumeration = "DisguiseType", tag = "6")]
    pub disguise_type: i32,
    #[prost(string, repeated, tag = "7")]
    pub condition: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "8")]
    pub cost: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisguiseMasters {
    #[prost(message, repeated, tag = "1")]
    pub disguise_masters: ::prost::alloc::vec::Vec<DisguiseMaster>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DisguiseType {
    InvalidDisguiseType = 0,
    DisguiseSkin = 1,
    DisguiseAction = 2,
    DisguiseObject = 3,
}
impl DisguiseType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DisguiseType::InvalidDisguiseType => "INVALID_DISGUISE_TYPE",
            DisguiseType::DisguiseSkin => "DISGUISE_SKIN",
            DisguiseType::DisguiseAction => "DISGUISE_ACTION",
            DisguiseType::DisguiseObject => "DISGUISE_OBJECT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INVALID_DISGUISE_TYPE" => Some(Self::InvalidDisguiseType),
            "DISGUISE_SKIN" => Some(Self::DisguiseSkin),
            "DISGUISE_ACTION" => Some(Self::DisguiseAction),
            "DISGUISE_OBJECT" => Some(Self::DisguiseObject),
            _ => None,
        }
    }
}
