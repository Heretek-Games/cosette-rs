#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GlobalMail {
    #[prost(string, tag = "1")]
    pub global_mail_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub customized_sender_name: ::prost::alloc::string::String,
    #[prost(int64, tag = "11")]
    pub player_level: i64,
    #[prost(int64, repeated, tag = "12")]
    pub server_ids: ::prost::alloc::vec::Vec<i64>,
    #[prost(string, repeated, tag = "13")]
    pub affcodes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "21")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "22")]
    pub text_content: ::prost::alloc::string::String,
    #[prost(string, tag = "23")]
    pub image_url: ::prost::alloc::string::String,
    #[prost(string, tag = "24")]
    pub external_url: ::prost::alloc::string::String,
    #[prost(string, tag = "25")]
    pub external_url_desc: ::prost::alloc::string::String,
    #[prost(string, tag = "26")]
    pub function_link: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "27")]
    pub rewards: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(sint64, tag = "28")]
    pub mail_sending_time: i64,
    #[prost(sint64, tag = "29")]
    pub mail_expiring_time: i64,
    #[prost(int64, tag = "30")]
    pub life_time_before_read_in_hour: i64,
    #[prost(int64, tag = "31")]
    pub life_time_after_read_in_hour: i64,
    #[prost(string, tag = "32")]
    pub ja_title: ::prost::alloc::string::String,
    #[prost(string, tag = "33")]
    pub ja_content: ::prost::alloc::string::String,
    #[prost(string, tag = "34")]
    pub zh_tw_title: ::prost::alloc::string::String,
    #[prost(string, tag = "35")]
    pub zh_tw_content: ::prost::alloc::string::String,
    #[prost(string, tag = "36")]
    pub en_title: ::prost::alloc::string::String,
    #[prost(string, tag = "37")]
    pub en_content: ::prost::alloc::string::String,
    #[prost(string, tag = "38")]
    pub ko_title: ::prost::alloc::string::String,
    #[prost(string, tag = "39")]
    pub ko_content: ::prost::alloc::string::String,
    #[prost(string, tag = "40")]
    pub th_title: ::prost::alloc::string::String,
    #[prost(string, tag = "41")]
    pub th_content: ::prost::alloc::string::String,
    #[prost(bool, tag = "42")]
    pub after_send_time_cant_receive: bool,
    #[prost(sint64, tag = "91")]
    pub created_at: i64,
    #[prost(sint64, tag = "92")]
    pub updated_at: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerMail {
    #[prost(string, tag = "1")]
    pub player_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub mail_id: ::prost::alloc::string::String,
    #[prost(enumeration = "SenderType", tag = "3")]
    pub sender_type: i32,
    #[prost(string, tag = "4")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub sender_name: ::prost::alloc::string::String,
    #[prost(string, tag = "21")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "22")]
    pub text_content: ::prost::alloc::string::String,
    #[prost(string, tag = "23")]
    pub image_url: ::prost::alloc::string::String,
    #[prost(string, tag = "24")]
    pub external_url: ::prost::alloc::string::String,
    #[prost(string, tag = "25")]
    pub external_url_desc: ::prost::alloc::string::String,
    #[prost(string, tag = "26")]
    pub function_link: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "27")]
    pub rewards: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, tag = "30")]
    pub life_time_before_read_in_hour: i64,
    #[prost(int64, tag = "31")]
    pub life_time_after_read_in_hour: i64,
    #[prost(string, tag = "32")]
    pub ja_title: ::prost::alloc::string::String,
    #[prost(string, tag = "33")]
    pub ja_content: ::prost::alloc::string::String,
    #[prost(string, tag = "34")]
    pub zh_tw_title: ::prost::alloc::string::String,
    #[prost(string, tag = "35")]
    pub zh_tw_content: ::prost::alloc::string::String,
    #[prost(string, tag = "36")]
    pub en_title: ::prost::alloc::string::String,
    #[prost(string, tag = "37")]
    pub en_content: ::prost::alloc::string::String,
    #[prost(string, tag = "38")]
    pub ko_title: ::prost::alloc::string::String,
    #[prost(string, tag = "39")]
    pub ko_content: ::prost::alloc::string::String,
    #[prost(string, tag = "40")]
    pub th_title: ::prost::alloc::string::String,
    #[prost(string, tag = "41")]
    pub th_content: ::prost::alloc::string::String,
    #[prost(sint64, tag = "51")]
    pub mail_received_time: i64,
    #[prost(sint64, tag = "52")]
    pub life_time_end_at: i64,
    #[prost(sint64, tag = "61")]
    pub read_at: i64,
    #[prost(sint64, tag = "62")]
    pub deleted_at: i64,
    #[prost(sint64, tag = "63")]
    pub rewards_received_at: i64,
    #[prost(sint64, tag = "91")]
    pub created_at: i64,
    #[prost(sint64, tag = "92")]
    pub updated_at: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SenderType {
    InvalidSenderType = 0,
    GlobalMailType = 1,
    RewardsMail = 2,
    PersonalMailType = 3,
}
impl SenderType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SenderType::InvalidSenderType => "INVALID_SENDER_TYPE",
            SenderType::GlobalMailType => "GLOBAL_MAIL_TYPE",
            SenderType::RewardsMail => "REWARDS_MAIL",
            SenderType::PersonalMailType => "PERSONAL_MAIL_TYPE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INVALID_SENDER_TYPE" => Some(Self::InvalidSenderType),
            "GLOBAL_MAIL_TYPE" => Some(Self::GlobalMailType),
            "REWARDS_MAIL" => Some(Self::RewardsMail),
            "PERSONAL_MAIL_TYPE" => Some(Self::PersonalMailType),
            _ => None,
        }
    }
}
