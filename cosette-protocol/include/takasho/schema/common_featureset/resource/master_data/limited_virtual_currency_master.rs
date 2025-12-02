#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LimitedVirtualCurrencyMasters {
    #[prost(message, repeated, tag = "1")]
    pub limited_virtual_currency_masters: ::prost::alloc::vec::Vec<
        LimitedVirtualCurrencyMaster,
    >,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LimitedVirtualCurrencyMaster {
    #[prost(string, tag = "1")]
    pub virtual_currency_key: ::prost::alloc::string::String,
    #[prost(sint64, tag = "3")]
    pub expires_in: i64,
    #[prost(enumeration = "limited_virtual_currency_master::ExpirationUnit", tag = "2")]
    pub expiration_unit: i32,
    #[prost(
        enumeration = "limited_virtual_currency_master::ExpirationPeriod",
        tag = "4"
    )]
    pub expiration_period: i32,
}
/// Nested message and enum types in `LimitedVirtualCurrencyMaster`.
pub mod limited_virtual_currency_master {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum ExpirationUnit {
        Day = 0,
        Month = 1,
    }
    impl ExpirationUnit {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ExpirationUnit::Day => "DAY",
                ExpirationUnit::Month => "MONTH",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DAY" => Some(Self::Day),
                "MONTH" => Some(Self::Month),
                _ => None,
            }
        }
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum ExpirationPeriod {
        OnTime = 0,
        EndOfMonth = 1,
    }
    impl ExpirationPeriod {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ExpirationPeriod::OnTime => "ON_TIME",
                ExpirationPeriod::EndOfMonth => "END_OF_MONTH",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ON_TIME" => Some(Self::OnTime),
                "END_OF_MONTH" => Some(Self::EndOfMonth),
                _ => None,
            }
        }
    }
}
