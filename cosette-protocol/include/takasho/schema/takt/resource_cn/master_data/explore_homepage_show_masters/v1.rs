#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExploreHomepageShowMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub explore_homepage_show_master_id: i64,
    #[prost(enumeration = "HomepageShowCharacterType", tag = "4")]
    pub character_type: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExploreHomepageShowMasters {
    #[prost(message, repeated, tag = "1")]
    pub explore_homepage_show_masters: ::prost::alloc::vec::Vec<
        ExploreHomepageShowMaster,
    >,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum HomepageShowCharacterType {
    HomepageShowCharacterInvalidType = 0,
    HomepageShowCharacterNpcType = 1,
    HomepageShowCharacterCommonType = 2,
}
impl HomepageShowCharacterType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            HomepageShowCharacterType::HomepageShowCharacterInvalidType => {
                "HomepageShowCharacterInvalidType"
            }
            HomepageShowCharacterType::HomepageShowCharacterNpcType => {
                "HomepageShowCharacterNPCType"
            }
            HomepageShowCharacterType::HomepageShowCharacterCommonType => {
                "HomepageShowCharacterCommonType"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "HomepageShowCharacterInvalidType" => {
                Some(Self::HomepageShowCharacterInvalidType)
            }
            "HomepageShowCharacterNPCType" => Some(Self::HomepageShowCharacterNpcType),
            "HomepageShowCharacterCommonType" => {
                Some(Self::HomepageShowCharacterCommonType)
            }
            _ => None,
        }
    }
}
