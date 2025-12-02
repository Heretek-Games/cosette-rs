#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Criterion {
    #[prost(int64, tag = "1")]
    pub count: i64,
    #[prost(enumeration = "Route", repeated, tag = "2")]
    pub routes: ::prost::alloc::vec::Vec<i32>,
    #[prost(string, repeated, tag = "3")]
    pub search_labels: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
    #[prost(sint64, tag = "5")]
    pub expired_at_before: i64,
    #[prost(sint64, tag = "6")]
    pub expired_at_after: i64,
    #[prost(message, optional, tag = "7")]
    pub order: ::core::option::Option<Order>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Order {
    #[prost(enumeration = "OrderField", tag = "1")]
    pub field: i32,
    #[prost(enumeration = "OrderDirection", tag = "2")]
    pub direction: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerInventory {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub player_id: ::prost::alloc::string::String,
    #[prost(enumeration = "ItemType", tag = "3")]
    pub item_type: i32,
    #[prost(string, tag = "4")]
    pub schema_key: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "5")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration = "Route", tag = "6")]
    pub route: i32,
    #[prost(string, tag = "7")]
    pub message: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub search_label: ::prost::alloc::string::String,
    #[prost(sint64, tag = "9")]
    pub opened_at: i64,
    #[prost(sint64, tag = "10")]
    pub expired_at: i64,
    #[prost(string, tag = "11")]
    pub system_resource_name: ::prost::alloc::string::String,
    #[prost(sint64, tag = "12")]
    pub system_resource_num: i64,
    #[prost(sint64, tag = "13")]
    pub created_at: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReceivedPlayerInventory {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub original_inventory_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub player_id: ::prost::alloc::string::String,
    #[prost(enumeration = "ItemType", tag = "4")]
    pub item_type: i32,
    #[prost(string, tag = "5")]
    pub schema_key: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "6")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration = "Route", tag = "7")]
    pub route: i32,
    #[prost(string, tag = "8")]
    pub message: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub search_label: ::prost::alloc::string::String,
    #[prost(sint64, tag = "10")]
    pub opened_at: i64,
    #[prost(sint64, tag = "11")]
    pub expired_at: i64,
    #[prost(string, tag = "12")]
    pub system_resource_name: ::prost::alloc::string::String,
    #[prost(sint64, tag = "13")]
    pub system_resource_num: i64,
    #[prost(sint64, tag = "14")]
    pub created_at: i64,
    #[prost(sint64, tag = "15")]
    pub received_at: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ItemType {
    GameRuntime = 0,
    VirtualCurrency = 1,
    PlayerKeyValueStore = 2,
}
impl ItemType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ItemType::GameRuntime => "GAME_RUNTIME",
            ItemType::VirtualCurrency => "VIRTUAL_CURRENCY",
            ItemType::PlayerKeyValueStore => "PLAYER_KEY_VALUE_STORE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "GAME_RUNTIME" => Some(Self::GameRuntime),
            "VIRTUAL_CURRENCY" => Some(Self::VirtualCurrency),
            "PLAYER_KEY_VALUE_STORE" => Some(Self::PlayerKeyValueStore),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Route {
    Unknown = 0,
    Purchase = 1,
    LoginBonus = 2,
    LootBox = 3,
    Achievement = 4,
    Compensation = 5,
    GameProduct = 6,
    BoxLootBox = 7,
    StepUpLootBox = 8,
    TotalLoginBonus = 9,
    OpFundUnlock = 10,
}
impl Route {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Route::Unknown => "UNKNOWN",
            Route::Purchase => "PURCHASE",
            Route::LoginBonus => "LOGIN_BONUS",
            Route::LootBox => "LOOT_BOX",
            Route::Achievement => "ACHIEVEMENT",
            Route::Compensation => "COMPENSATION",
            Route::GameProduct => "GAME_PRODUCT",
            Route::BoxLootBox => "BOX_LOOT_BOX",
            Route::StepUpLootBox => "STEP_UP_LOOT_BOX",
            Route::TotalLoginBonus => "TOTAL_LOGIN_BONUS",
            Route::OpFundUnlock => "OP_FUND_UNLOCK",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN" => Some(Self::Unknown),
            "PURCHASE" => Some(Self::Purchase),
            "LOGIN_BONUS" => Some(Self::LoginBonus),
            "LOOT_BOX" => Some(Self::LootBox),
            "ACHIEVEMENT" => Some(Self::Achievement),
            "COMPENSATION" => Some(Self::Compensation),
            "GAME_PRODUCT" => Some(Self::GameProduct),
            "BOX_LOOT_BOX" => Some(Self::BoxLootBox),
            "STEP_UP_LOOT_BOX" => Some(Self::StepUpLootBox),
            "TOTAL_LOGIN_BONUS" => Some(Self::TotalLoginBonus),
            "OP_FUND_UNLOCK" => Some(Self::OpFundUnlock),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrderField {
    OpenedAt = 0,
    ExpiredAt = 1,
}
impl OrderField {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OrderField::OpenedAt => "OPENED_AT",
            OrderField::ExpiredAt => "EXPIRED_AT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OPENED_AT" => Some(Self::OpenedAt),
            "EXPIRED_AT" => Some(Self::ExpiredAt),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrderDirection {
    Asc = 0,
    Desc = 1,
}
impl OrderDirection {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OrderDirection::Asc => "ASC",
            OrderDirection::Desc => "DESC",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ASC" => Some(Self::Asc),
            "DESC" => Some(Self::Desc),
            _ => None,
        }
    }
}
