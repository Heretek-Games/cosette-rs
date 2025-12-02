#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DropMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub drop_master_id: i64,
    #[prost(string, repeated, tag = "3")]
    pub fixed_drop: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "4")]
    pub random_drop: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "5")]
    pub random_drop_num: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(enumeration = "DropType", tag = "6")]
    pub drop_type: i32,
    #[prost(int64, tag = "7")]
    pub pick_num: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DropMasters {
    #[prost(message, repeated, tag = "1")]
    pub drop_masters: ::prost::alloc::vec::Vec<DropMaster>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DropType {
    FixedDrop = 0,
    PutBackDrop = 1,
    NoPutBackDrop = 2,
    PickDrop = 3,
}
impl DropType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DropType::FixedDrop => "FIXED_DROP",
            DropType::PutBackDrop => "PUT_BACK_DROP",
            DropType::NoPutBackDrop => "NO_PUT_BACK_DROP",
            DropType::PickDrop => "PICK_DROP",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "FIXED_DROP" => Some(Self::FixedDrop),
            "PUT_BACK_DROP" => Some(Self::PutBackDrop),
            "NO_PUT_BACK_DROP" => Some(Self::NoPutBackDrop),
            "PICK_DROP" => Some(Self::PickDrop),
            _ => None,
        }
    }
}
