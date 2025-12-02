#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Realms {
    #[prost(message, repeated, tag = "1")]
    pub realms: ::prost::alloc::vec::Vec<
        super::super::super::super::common_featureset::resource::realm::v1::Realm,
    >,
}
