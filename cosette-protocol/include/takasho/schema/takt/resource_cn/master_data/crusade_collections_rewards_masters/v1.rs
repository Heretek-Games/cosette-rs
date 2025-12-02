#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CrusadeCollectionsRewardsMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub crusade_collections_rewards_master_id: i64,
    #[prost(int64, repeated, tag = "3")]
    pub master_ids: ::prost::alloc::vec::Vec<i64>,
    #[prost(enumeration = "CrusadeCollectionTypeEnum", tag = "4")]
    pub r#type: i32,
    #[prost(string, repeated, tag = "5")]
    pub rewards: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CrusadeCollectionsRewardsMasters {
    #[prost(message, repeated, tag = "1")]
    pub crusade_collections_rewards_masters: ::prost::alloc::vec::Vec<
        CrusadeCollectionsRewardsMaster,
    >,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CrusadeCollectionTypeEnum {
    CrusadeCollectionTypeInvalid = 0,
    CrusadeCollectionTypeTreasure = 1,
    CrusadeCollectionTypeEvent = 2,
}
impl CrusadeCollectionTypeEnum {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CrusadeCollectionTypeEnum::CrusadeCollectionTypeInvalid => {
                "CrusadeCollectionTypeInvalid"
            }
            CrusadeCollectionTypeEnum::CrusadeCollectionTypeTreasure => {
                "CrusadeCollectionTypeTreasure"
            }
            CrusadeCollectionTypeEnum::CrusadeCollectionTypeEvent => {
                "CrusadeCollectionTypeEvent"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CrusadeCollectionTypeInvalid" => Some(Self::CrusadeCollectionTypeInvalid),
            "CrusadeCollectionTypeTreasure" => Some(Self::CrusadeCollectionTypeTreasure),
            "CrusadeCollectionTypeEvent" => Some(Self::CrusadeCollectionTypeEvent),
            _ => None,
        }
    }
}
