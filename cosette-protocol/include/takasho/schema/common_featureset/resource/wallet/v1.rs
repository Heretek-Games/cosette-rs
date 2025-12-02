#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Wallet {
    #[prost(string, tag = "1")]
    pub virtual_currency_name: ::prost::alloc::string::String,
    #[prost(enumeration = "wallet::Platform", tag = "2")]
    pub platform: i32,
    #[prost(message, optional, tag = "3")]
    pub balance: ::core::option::Option<wallet::Balance>,
}
/// Nested message and enum types in `Wallet`.
pub mod wallet {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Balance {
        #[prost(sint64, tag = "1")]
        pub free: i64,
        #[prost(sint64, tag = "2")]
        pub paid: i64,
        #[prost(sint64, tag = "3")]
        pub total: i64,
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
    pub enum Platform {
        None = 0,
        Apple = 1,
        Google = 2,
    }
    impl Platform {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Platform::None => "NONE",
                Platform::Apple => "APPLE",
                Platform::Google => "GOOGLE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "NONE" => Some(Self::None),
                "APPLE" => Some(Self::Apple),
                "GOOGLE" => Some(Self::Google),
                _ => None,
            }
        }
    }
}
