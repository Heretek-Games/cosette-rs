#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpEvent {
    #[prost(string, tag = "1")]
    pub op_event_id: ::prost::alloc::string::String,
    #[prost(enumeration = "OpEventType", tag = "2")]
    pub op_event_type: i32,
    #[prost(int64, tag = "3")]
    pub open_after_player_elapsed_time_seconds: i64,
    #[prost(int64, tag = "4")]
    pub close_after_player_elapsed_time_seconds: i64,
    #[prost(int64, tag = "5")]
    pub player_level: i64,
    #[prost(string, repeated, tag = "6")]
    pub objective_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, tag = "7")]
    pub open_after_server_elapsed_time_seconds: i64,
    #[prost(int64, tag = "8")]
    pub close_after_server_elapsed_time_seconds: i64,
    #[prost(string, tag = "9")]
    pub transaction_payload: ::prost::alloc::string::String,
    #[prost(sint64, tag = "10")]
    pub opened_at: i64,
    #[prost(sint64, tag = "11")]
    pub closed_at: i64,
    #[prost(message, repeated, tag = "12")]
    pub consume_resource_sets: ::prost::alloc::vec::Vec<
        super::super::consume_resource_set::v1::ConsumeResourceSet,
    >,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OpEventType {
    UnknownType = 0,
    Fund = 1,
}
impl OpEventType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OpEventType::UnknownType => "UNKNOWN_TYPE",
            OpEventType::Fund => "FUND",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN_TYPE" => Some(Self::UnknownType),
            "FUND" => Some(Self::Fund),
            _ => None,
        }
    }
}
