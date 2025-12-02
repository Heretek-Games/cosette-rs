#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerDeviceBinding {
    #[prost(string, tag = "1")]
    pub player_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub device_id: ::prost::alloc::string::String,
    #[prost(sint64, tag = "91")]
    pub created_at: i64,
    #[prost(sint64, tag = "92")]
    pub updated_at: i64,
}
