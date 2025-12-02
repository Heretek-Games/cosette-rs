#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StaminaPurchaseMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub stamina_purchase_master_id: i64,
    #[prost(int64, tag = "3")]
    pub purchase_count: i64,
    #[prost(enumeration = "CostType", tag = "24")]
    pub cost_type: i32,
    #[prost(string, repeated, tag = "4")]
    pub require_content: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, tag = "5")]
    pub recover_count: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StaminaPurchaseMasters {
    #[prost(message, repeated, tag = "1")]
    pub stamina_purchase_masters: ::prost::alloc::vec::Vec<StaminaPurchaseMaster>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CostType {
    Stone = 0,
    Stone1 = 1,
    Gem = 2,
}
impl CostType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CostType::Stone => "STONE",
            CostType::Stone1 => "STONE_1",
            CostType::Gem => "GEM",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "STONE" => Some(Self::Stone),
            "STONE_1" => Some(Self::Stone1),
            "GEM" => Some(Self::Gem),
            _ => None,
        }
    }
}
