#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ReceivableTimeType {
    Default = 0,
    DateLine = 1,
    Unlimited = 2,
}
impl ReceivableTimeType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ReceivableTimeType::Default => "DEFAULT",
            ReceivableTimeType::DateLine => "DATE_LINE",
            ReceivableTimeType::Unlimited => "UNLIMITED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DEFAULT" => Some(Self::Default),
            "DATE_LINE" => Some(Self::DateLine),
            "UNLIMITED" => Some(Self::Unlimited),
            _ => None,
        }
    }
}
