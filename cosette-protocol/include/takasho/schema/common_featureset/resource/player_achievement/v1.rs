#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerAchievement {
    #[prost(message, optional, tag = "1")]
    pub achievement: ::core::option::Option<super::super::achievement::v1::Achievement>,
    #[prost(sint64, tag = "2")]
    pub last_unlocked_at: i64,
}
