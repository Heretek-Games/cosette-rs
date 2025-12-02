#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PuzzleExploreMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub puzzle_explore_master_id: i64,
    #[prost(enumeration = "PuzzleType", tag = "3")]
    pub type_id: i32,
    #[prost(int64, tag = "4")]
    pub battle_character_master_id: i64,
    #[prost(string, repeated, tag = "5")]
    pub first_rewards: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "6")]
    pub win_rewards: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "7")]
    pub lose_rewards: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "8")]
    pub consume_contents: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PuzzleExploreMasters {
    #[prost(message, repeated, tag = "1")]
    pub puzzle_explore_masters: ::prost::alloc::vec::Vec<PuzzleExploreMaster>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PuzzleType {
    InvalidPuzzleType = 0,
    PushUp = 1,
    Fishing = 2,
    CardPlaying = 13,
    DrinkMaking = 15,
    GachaMachine = 16,
}
impl PuzzleType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PuzzleType::InvalidPuzzleType => "InvalidPuzzleType",
            PuzzleType::PushUp => "PushUp",
            PuzzleType::Fishing => "Fishing",
            PuzzleType::CardPlaying => "CardPlaying",
            PuzzleType::DrinkMaking => "DrinkMaking",
            PuzzleType::GachaMachine => "GachaMachine",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "InvalidPuzzleType" => Some(Self::InvalidPuzzleType),
            "PushUp" => Some(Self::PushUp),
            "Fishing" => Some(Self::Fishing),
            "CardPlaying" => Some(Self::CardPlaying),
            "DrinkMaking" => Some(Self::DrinkMaking),
            "GachaMachine" => Some(Self::GachaMachine),
            _ => None,
        }
    }
}
