#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Wallet {
    #[prost(message, repeated, tag = "1")]
    pub virtual_currencies: ::prost::alloc::vec::Vec<wallet::VirtualCurrency>,
    #[prost(message, repeated, tag = "2")]
    pub player_key_values: ::prost::alloc::vec::Vec<wallet::PlayerKeyValue>,
}
/// Nested message and enum types in `Wallet`.
pub mod wallet {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct VirtualCurrency {
        #[prost(string, tag = "1")]
        pub virtual_currency_name: ::prost::alloc::string::String,
        #[prost(enumeration = "Platform", tag = "2")]
        pub platform: i32,
        #[prost(sint64, tag = "3")]
        pub amount: i64,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PlayerKeyValue {
        #[prost(string, tag = "1")]
        pub key: ::prost::alloc::string::String,
        #[prost(sint64, tag = "2")]
        pub amount: i64,
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
