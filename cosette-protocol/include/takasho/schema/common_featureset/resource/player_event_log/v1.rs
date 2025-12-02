#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerEventLog {
    #[prost(string, tag = "1")]
    pub event_category: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub event_id: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "3")]
    pub payload: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "4")]
    pub player_state: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LootBoxV3 {
    #[prost(string, tag = "1")]
    pub transaction_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub loot_box_product_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    pub num: i64,
    #[prost(string, repeated, tag = "4")]
    pub content_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "5")]
    pub consume_resources: ::prost::alloc::vec::Vec<ConsumeResource>,
    #[prost(bytes = "vec", repeated, tag = "6")]
    pub content_values: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StepUpLootBoxV2 {
    #[prost(string, tag = "1")]
    pub transaction_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub step_up_loot_box_product_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    pub step: i64,
    #[prost(int64, tag = "4")]
    pub next_step: i64,
    #[prost(string, repeated, tag = "5")]
    pub content_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "6")]
    pub consume_resources: ::prost::alloc::vec::Vec<ConsumeResource>,
    #[prost(bytes = "vec", repeated, tag = "7")]
    pub content_values: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConsumeResource {
    #[prost(string, tag = "1")]
    pub consume_resource_key: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub consume_resource_num: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FriendV1 {
    #[prost(enumeration = "friend_v1::ActionType", tag = "1")]
    pub action_type: i32,
    #[prost(string, repeated, tag = "2")]
    pub target_player_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, tag = "3")]
    pub friends_num: i64,
    #[prost(int64, tag = "4")]
    pub max_friends_num: i64,
}
/// Nested message and enum types in `FriendV1`.
pub mod friend_v1 {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum ActionType {
        Send = 0,
        Approve = 1,
        Reject = 2,
        DeleteFriend = 3,
        DeleteSentRequest = 4,
    }
    impl ActionType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ActionType::Send => "SEND",
                ActionType::Approve => "APPROVE",
                ActionType::Reject => "REJECT",
                ActionType::DeleteFriend => "DELETE_FRIEND",
                ActionType::DeleteSentRequest => "DELETE_SENT_REQUEST",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SEND" => Some(Self::Send),
                "APPROVE" => Some(Self::Approve),
                "REJECT" => Some(Self::Reject),
                "DELETE_FRIEND" => Some(Self::DeleteFriend),
                "DELETE_SENT_REQUEST" => Some(Self::DeleteSentRequest),
                _ => None,
            }
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConsumeVc {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub currencies: ::prost::alloc::vec::Vec<consume_vc::Currency>,
}
/// Nested message and enum types in `ConsumeVc`.
pub mod consume_vc {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Currency {
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        #[prost(sint64, tag = "2")]
        pub amount: i64,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BoxLootBox {
    #[prost(string, tag = "1")]
    pub transaction_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub gacha_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    pub exec_num: i64,
    #[prost(string, repeated, tag = "4")]
    pub get_item_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "5")]
    pub paid_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "6")]
    pub paid_amount: i64,
    #[prost(int64, tag = "7")]
    pub af_box_remain: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SystemAuthorizeV3 {
    #[prost(string, tag = "1")]
    pub player_id: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub new_player: bool,
}
