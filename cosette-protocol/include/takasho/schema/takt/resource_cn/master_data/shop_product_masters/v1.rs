#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShopProductMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub shop_product_master_id: i64,
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    #[prost(int64, tag = "4")]
    pub content_id: i64,
    #[prost(int64, tag = "5")]
    pub content_amount: i64,
    #[prost(int64, tag = "6")]
    pub shop_category_master_id: i64,
    #[prost(int64, tag = "7")]
    pub shop_master_id: i64,
    #[prost(int64, tag = "8")]
    pub former_product_id: i64,
    #[prost(bool, tag = "9")]
    pub is_former_product: bool,
    #[prost(enumeration = "CostType", tag = "11")]
    pub cost_type: i32,
    #[prost(int64, tag = "12")]
    pub cost_id: i64,
    #[prost(int64, tag = "13")]
    pub labeled_price: i64,
    #[prost(int64, tag = "14")]
    pub discounted_price: i64,
    #[prost(string, tag = "15")]
    pub discount: ::prost::alloc::string::String,
    #[prost(enumeration = "PurchaseCountLimitType", tag = "21")]
    pub purchase_count_limit_type: i32,
    #[prost(int64, tag = "22")]
    pub purchase_limit: i64,
    #[prost(enumeration = "ConstrainType", tag = "23")]
    pub constrain_type: i32,
    #[prost(int64, tag = "24")]
    pub constrain_value: i64,
    #[prost(int64, tag = "25")]
    pub sort_id: i64,
    #[prost(string, tag = "26")]
    pub start_time: ::prost::alloc::string::String,
    #[prost(string, tag = "27")]
    pub end_time: ::prost::alloc::string::String,
    #[prost(string, tag = "28")]
    pub lcx_product_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "29")]
    pub gift_diamond: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, tag = "31")]
    pub rebate_content: i64,
    #[prost(string, tag = "30")]
    pub extra_cost: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "40")]
    pub extra_cost1: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "50")]
    pub extra_prize: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShopProductMasters {
    #[prost(message, repeated, tag = "1")]
    pub shop_product_masters: ::prost::alloc::vec::Vec<ShopProductMaster>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CostType {
    InvalidCostType = 0,
    Content = 1,
    Money = 2,
}
impl CostType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CostType::InvalidCostType => "Invalid_CostType",
            CostType::Content => "CostType_Content",
            CostType::Money => "CostType_Money",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Invalid_CostType" => Some(Self::InvalidCostType),
            "CostType_Content" => Some(Self::Content),
            "CostType_Money" => Some(Self::Money),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PurchaseCountLimitType {
    InvalidLimitType = 0,
    LimitTypeDaily = 1,
    LimitTypeWeekly = 2,
    LimitTypeMonthly = 3,
    LimitTypePerAccount = 4,
    LimitTypeNoLimit = 5,
}
impl PurchaseCountLimitType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PurchaseCountLimitType::InvalidLimitType => "Invalid_LimitType",
            PurchaseCountLimitType::LimitTypeDaily => "LimitType_Daily",
            PurchaseCountLimitType::LimitTypeWeekly => "LimitType_Weekly",
            PurchaseCountLimitType::LimitTypeMonthly => "LimitType_Monthly",
            PurchaseCountLimitType::LimitTypePerAccount => "LimitType_PerAccount",
            PurchaseCountLimitType::LimitTypeNoLimit => "LimitType_NoLimit",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Invalid_LimitType" => Some(Self::InvalidLimitType),
            "LimitType_Daily" => Some(Self::LimitTypeDaily),
            "LimitType_Weekly" => Some(Self::LimitTypeWeekly),
            "LimitType_Monthly" => Some(Self::LimitTypeMonthly),
            "LimitType_PerAccount" => Some(Self::LimitTypePerAccount),
            "LimitType_NoLimit" => Some(Self::LimitTypeNoLimit),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ConstrainType {
    InvalidConstrainType = 0,
    PlayerLevel = 1,
    Pve = 2,
    MemoryQuest = 3,
    NotRequired = 4,
    ExploreEvent = 5,
    BattleCharacter = 6,
}
impl ConstrainType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ConstrainType::InvalidConstrainType => "Invalid_ConstrainType",
            ConstrainType::PlayerLevel => "ConstrainType_PlayerLevel",
            ConstrainType::Pve => "ConstrainType_Pve",
            ConstrainType::MemoryQuest => "ConstrainType_MemoryQuest",
            ConstrainType::NotRequired => "ConstrainType_NotRequired",
            ConstrainType::ExploreEvent => "ConstrainType_ExploreEvent",
            ConstrainType::BattleCharacter => "ConstrainType_BattleCharacter",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Invalid_ConstrainType" => Some(Self::InvalidConstrainType),
            "ConstrainType_PlayerLevel" => Some(Self::PlayerLevel),
            "ConstrainType_Pve" => Some(Self::Pve),
            "ConstrainType_MemoryQuest" => Some(Self::MemoryQuest),
            "ConstrainType_NotRequired" => Some(Self::NotRequired),
            "ConstrainType_ExploreEvent" => Some(Self::ExploreEvent),
            "ConstrainType_BattleCharacter" => Some(Self::BattleCharacter),
            _ => None,
        }
    }
}
