#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Value {
    #[prost(string, tag = "1")]
    pub value_key: ::prost::alloc::string::String,
    #[prost(enumeration = "Type", tag = "2")]
    pub r#type: i32,
    #[prost(string, tag = "3")]
    pub id: ::prost::alloc::string::String,
    #[prost(sint64, tag = "4")]
    pub count: i64,
    #[prost(string, tag = "5")]
    pub name: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Type {
    InvalidType = 0,
    BattleCharacter = 1,
    Equipment = 2,
    Item = 3,
    TrainingRoom = 4,
    Keyword = 5,
    Max = 6,
}
impl Type {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Type::InvalidType => "INVALID_TYPE",
            Type::BattleCharacter => "BATTLE_CHARACTER",
            Type::Equipment => "EQUIPMENT",
            Type::Item => "ITEM",
            Type::TrainingRoom => "TRAINING_ROOM",
            Type::Keyword => "KEYWORD",
            Type::Max => "MAX",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INVALID_TYPE" => Some(Self::InvalidType),
            "BATTLE_CHARACTER" => Some(Self::BattleCharacter),
            "EQUIPMENT" => Some(Self::Equipment),
            "ITEM" => Some(Self::Item),
            "TRAINING_ROOM" => Some(Self::TrainingRoom),
            "KEYWORD" => Some(Self::Keyword),
            "MAX" => Some(Self::Max),
            _ => None,
        }
    }
}
