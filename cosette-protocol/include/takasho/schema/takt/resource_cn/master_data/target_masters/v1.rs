#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub target_master_id: i64,
    #[prost(int64, repeated, tag = "11")]
    pub previous_target_ids: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, tag = "3")]
    pub target_category: i64,
    #[prost(int64, tag = "4")]
    pub parent_id: i64,
    #[prost(string, repeated, tag = "5")]
    pub unlock_conditions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, tag = "6")]
    pub target_type_id: i64,
    #[prost(int64, tag = "7")]
    pub mission_type_value1: i64,
    #[prost(int64, tag = "8")]
    pub mission_type_value2: i64,
    #[prost(int64, tag = "9")]
    pub mission_type_value3: i64,
    #[prost(string, repeated, tag = "10")]
    pub rewards: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, tag = "12")]
    pub required_total_index: i64,
    #[prost(int64, repeated, tag = "13")]
    pub mission_type_value4: ::prost::alloc::vec::Vec<i64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetMasters {
    #[prost(message, repeated, tag = "1")]
    pub target_masters: ::prost::alloc::vec::Vec<TargetMaster>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TargetCategory {
    InvalidCategory = 0,
    FatherCategory = 1,
    ChildCategory = 2,
}
impl TargetCategory {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TargetCategory::InvalidCategory => "INVALID_CATEGORY",
            TargetCategory::FatherCategory => "FATHER_CATEGORY",
            TargetCategory::ChildCategory => "CHILD_CATEGORY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INVALID_CATEGORY" => Some(Self::InvalidCategory),
            "FATHER_CATEGORY" => Some(Self::FatherCategory),
            "CHILD_CATEGORY" => Some(Self::ChildCategory),
            _ => None,
        }
    }
}
