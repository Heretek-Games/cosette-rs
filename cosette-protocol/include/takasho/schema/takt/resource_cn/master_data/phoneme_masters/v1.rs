#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhonemeMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub phoneme_master_id: i64,
    #[prost(string, tag = "99")]
    pub name: ::prost::alloc::string::String,
    #[prost(enumeration = "Rarity", tag = "3")]
    pub rarity: i32,
    #[prost(int64, tag = "4")]
    pub max_level: i64,
    #[prost(int64, tag = "6")]
    pub exp_supply: i64,
    #[prost(string, repeated, tag = "50")]
    pub attributes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, tag = "55")]
    pub r#type: i64,
    #[prost(string, repeated, tag = "56")]
    pub entries_num: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, repeated, tag = "57")]
    pub entries_library: ::prost::alloc::vec::Vec<i64>,
    #[prost(string, repeated, tag = "58")]
    pub decomposition: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhonemeMasters {
    #[prost(message, repeated, tag = "1")]
    pub phoneme_masters: ::prost::alloc::vec::Vec<PhonemeMaster>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Rarity {
    InvalidRarity = 0,
    N = 1,
    R = 2,
    Sr = 3,
    Ssr = 4,
}
impl Rarity {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Rarity::InvalidRarity => "InvalidRarity",
            Rarity::N => "N",
            Rarity::R => "R",
            Rarity::Sr => "SR",
            Rarity::Ssr => "SSR",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "InvalidRarity" => Some(Self::InvalidRarity),
            "N" => Some(Self::N),
            "R" => Some(Self::R),
            "SR" => Some(Self::Sr),
            "SSR" => Some(Self::Ssr),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Type {
    InvalidColor = 0,
    Atk = 1,
    Def = 2,
    Oth = 3,
}
impl Type {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Type::InvalidColor => "InvalidColor",
            Type::Atk => "ATK",
            Type::Def => "DEF",
            Type::Oth => "OTH",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "InvalidColor" => Some(Self::InvalidColor),
            "ATK" => Some(Self::Atk),
            "DEF" => Some(Self::Def),
            "OTH" => Some(Self::Oth),
            _ => None,
        }
    }
}
