#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BattleCheckByIds {
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(int64, tag = "2")]
    pub battle_id: i64,
    #[prost(int64, tag = "3")]
    pub r#type: i64,
    #[prost(int64, tag = "4")]
    pub precision: i64,
    #[prost(bool, tag = "5")]
    pub is_always_open: bool,
    #[prost(bool, tag = "6")]
    pub is_always_close: bool,
    #[prost(int64, tag = "7")]
    pub check_method: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BattleCheckPrecision {
    None = 0,
    Basic = 1,
    MasterData = 2,
    Detail = 3,
    All = 99,
}
impl BattleCheckPrecision {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            BattleCheckPrecision::None => "NONE",
            BattleCheckPrecision::Basic => "Basic",
            BattleCheckPrecision::MasterData => "MasterData",
            BattleCheckPrecision::Detail => "Detail",
            BattleCheckPrecision::All => "ALL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NONE" => Some(Self::None),
            "Basic" => Some(Self::Basic),
            "MasterData" => Some(Self::MasterData),
            "Detail" => Some(Self::Detail),
            "ALL" => Some(Self::All),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BattleCheckMethod {
    Async = 0,
    Sync = 1,
}
impl BattleCheckMethod {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            BattleCheckMethod::Async => "Async",
            BattleCheckMethod::Sync => "Sync",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Async" => Some(Self::Async),
            "Sync" => Some(Self::Sync),
            _ => None,
        }
    }
}
