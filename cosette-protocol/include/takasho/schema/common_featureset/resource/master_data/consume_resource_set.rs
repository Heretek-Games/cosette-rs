#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConsumeResourceSet {
    #[prost(string, tag = "1")]
    pub consume_resource_set_id: ::prost::alloc::string::String,
    #[prost(enumeration = "ResourceType", tag = "2")]
    pub resource_type: i32,
    #[prost(sint64, tag = "3")]
    pub amount: i64,
    #[prost(message, repeated, tag = "4")]
    pub consume_resources: ::prost::alloc::vec::Vec<ConsumeResource>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConsumeResource {
    #[prost(string, tag = "1")]
    pub resource_key: ::prost::alloc::string::String,
    #[prost(sint64, tag = "2")]
    pub priority: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ResourceType {
    UnknownResource = 0,
    VirtualCurrency = 1,
    PlayerKeyValue = 2,
    NoConsume = 3,
}
impl ResourceType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ResourceType::UnknownResource => "UNKNOWN_RESOURCE",
            ResourceType::VirtualCurrency => "VIRTUAL_CURRENCY",
            ResourceType::PlayerKeyValue => "PLAYER_KEY_VALUE",
            ResourceType::NoConsume => "NO_CONSUME",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN_RESOURCE" => Some(Self::UnknownResource),
            "VIRTUAL_CURRENCY" => Some(Self::VirtualCurrency),
            "PLAYER_KEY_VALUE" => Some(Self::PlayerKeyValue),
            "NO_CONSUME" => Some(Self::NoConsume),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProductType {
    UnknownProduct = 0,
    GameProduct = 1,
    LootBox = 2,
    BoxLootBox = 3,
    StepUpLootBox = 4,
    OpEvent = 5,
}
impl ProductType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ProductType::UnknownProduct => "UNKNOWN_PRODUCT",
            ProductType::GameProduct => "GAME_PRODUCT",
            ProductType::LootBox => "LOOT_BOX",
            ProductType::BoxLootBox => "BOX_LOOT_BOX",
            ProductType::StepUpLootBox => "STEP_UP_LOOT_BOX",
            ProductType::OpEvent => "OP_EVENT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN_PRODUCT" => Some(Self::UnknownProduct),
            "GAME_PRODUCT" => Some(Self::GameProduct),
            "LOOT_BOX" => Some(Self::LootBox),
            "BOX_LOOT_BOX" => Some(Self::BoxLootBox),
            "STEP_UP_LOOT_BOX" => Some(Self::StepUpLootBox),
            "OP_EVENT" => Some(Self::OpEvent),
            _ => None,
        }
    }
}
