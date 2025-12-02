#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BattlePatchMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub battle_patch_master_id: i64,
    #[prost(enumeration = "PatchFixType", tag = "3")]
    pub hp_fix_type: i32,
    #[prost(int64, tag = "4")]
    pub hp_fix_value: i64,
    #[prost(enumeration = "PatchFixType", tag = "5")]
    pub atk_fix_type: i32,
    #[prost(int64, tag = "6")]
    pub atk_fix_value: i64,
    #[prost(enumeration = "PatchFixType", tag = "7")]
    pub def_fix_type: i32,
    #[prost(int64, tag = "8")]
    pub def_fix_value: i64,
    #[prost(enumeration = "PatchFixType", tag = "9")]
    pub int_fix_type: i32,
    #[prost(int64, tag = "10")]
    pub int_fix_value: i64,
    #[prost(enumeration = "PatchFixType", tag = "11")]
    pub res_fix_type: i32,
    #[prost(int64, tag = "12")]
    pub res_fix_value: i64,
    #[prost(enumeration = "PatchFixType", tag = "13")]
    pub dex_fix_type: i32,
    #[prost(int64, tag = "14")]
    pub dex_fix_value: i64,
    #[prost(enumeration = "PatchFixType", tag = "15")]
    pub crt_fix_type: i32,
    #[prost(int64, tag = "16")]
    pub crt_fix_value: i64,
    #[prost(enumeration = "PatchFixType", tag = "17")]
    pub crd_fix_type: i32,
    #[prost(int64, tag = "18")]
    pub crd_fix_value: i64,
    #[prost(enumeration = "PatchFixType", tag = "19")]
    pub acrt_fix_type: i32,
    #[prost(int64, tag = "20")]
    pub acrt_fix_value: i64,
    #[prost(enumeration = "PatchFixType", tag = "21")]
    pub crdr_fix_type: i32,
    #[prost(int64, tag = "22")]
    pub crdr_fix_value: i64,
    #[prost(enumeration = "PatchFixType", tag = "23")]
    pub ap_fix_type: i32,
    #[prost(int64, tag = "24")]
    pub ap_fix_value: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BattlePatchMasters {
    #[prost(message, repeated, tag = "1")]
    pub battle_patch_masters: ::prost::alloc::vec::Vec<BattlePatchMaster>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PatchFixType {
    None = 0,
    Add = 1,
    Minus = 2,
    Percentage = 3,
}
impl PatchFixType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PatchFixType::None => "NONE",
            PatchFixType::Add => "ADD",
            PatchFixType::Minus => "MINUS",
            PatchFixType::Percentage => "PERCENTAGE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NONE" => Some(Self::None),
            "ADD" => Some(Self::Add),
            "MINUS" => Some(Self::Minus),
            "PERCENTAGE" => Some(Self::Percentage),
            _ => None,
        }
    }
}
