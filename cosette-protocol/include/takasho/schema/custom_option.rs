#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerApiFileOption {
    #[prost(message, optional, tag = "1")]
    pub go_server: ::core::option::Option<player_api_file_option::GoServer>,
    #[prost(message, optional, tag = "2")]
    pub csharp_client: ::core::option::Option<player_api_file_option::CsharpClient>,
    #[prost(message, optional, tag = "3")]
    pub annotation: ::core::option::Option<player_api_file_option::Annotation>,
}
/// Nested message and enum types in `PlayerApiFileOption`.
pub mod player_api_file_option {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CsharpClient {
        #[prost(string, tag = "1")]
        pub endpoint_namespace: ::prost::alloc::string::String,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GoServer {
        #[prost(string, tag = "1")]
        pub package: ::prost::alloc::string::String,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Annotation {
        #[prost(bool, tag = "1")]
        pub debugging_feature: bool,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdminApiFileOption {
    #[prost(message, optional, tag = "1")]
    pub go_server: ::core::option::Option<admin_api_file_option::GoServer>,
    #[prost(message, optional, tag = "2")]
    pub annotation: ::core::option::Option<admin_api_file_option::Annotation>,
}
/// Nested message and enum types in `AdminApiFileOption`.
pub mod admin_api_file_option {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GoServer {
        #[prost(string, tag = "1")]
        pub package: ::prost::alloc::string::String,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Annotation {
        #[prost(bool, tag = "1")]
        pub debugging_feature: bool,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerApiMethodOption {
    #[prost(message, optional, tag = "1")]
    pub go_server: ::core::option::Option<player_api_method_option::GoServer>,
}
/// Nested message and enum types in `PlayerApiMethodOption`.
pub mod player_api_method_option {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GoServer {
        #[prost(string, tag = "1")]
        pub impl_package: ::prost::alloc::string::String,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdminApiMethodOption {
    #[prost(message, optional, tag = "1")]
    pub go_server: ::core::option::Option<admin_api_method_option::GoServer>,
}
/// Nested message and enum types in `AdminApiMethodOption`.
pub mod admin_api_method_option {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GoServer {
        #[prost(string, tag = "1")]
        pub impl_package: ::prost::alloc::string::String,
    }
}
