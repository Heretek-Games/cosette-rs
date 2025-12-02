#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerMiniMission {
    #[prost(string, tag = "1")]
    pub player_id: ::prost::alloc::string::String,
    #[prost(int64, repeated, tag = "30")]
    pub mini_mission_completed: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, repeated, tag = "40")]
    pub mini_mission_activated: ::prost::alloc::vec::Vec<i64>,
    #[prost(string, tag = "50")]
    pub mini_mission_datas: ::prost::alloc::string::String,
    #[prost(int64, repeated, tag = "60")]
    pub mini_mission_handling_characters: ::prost::alloc::vec::Vec<i64>,
    #[prost(string, repeated, tag = "70")]
    pub character_spawn_locations: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
}
