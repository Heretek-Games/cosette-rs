#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerQuestionnaire {
    #[prost(string, tag = "1")]
    pub player_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub questionnaire_id: ::prost::alloc::string::String,
    #[prost(sint64, tag = "3")]
    pub finished_at: i64,
    #[prost(string, tag = "11")]
    pub data_center_questionnaire_id: ::prost::alloc::string::String,
    #[prost(string, tag = "12")]
    pub group_id: ::prost::alloc::string::String,
    #[prost(sint64, tag = "91")]
    pub created_at: i64,
    #[prost(sint64, tag = "92")]
    pub updated_at: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GlobalQuestionnaire {
    #[prost(string, tag = "1")]
    pub questionnaire_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub affcodes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, repeated, tag = "3")]
    pub realm_ids: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, tag = "4")]
    pub days_after_server_opening: i64,
    #[prost(enumeration = "PlayerConstrainType", tag = "5")]
    pub player_constrain_type: i32,
    #[prost(int64, tag = "6")]
    pub player_constrain_type_value: i64,
    #[prost(sint64, tag = "7")]
    pub starting_time_in_second_accuracy: i64,
    #[prost(sint64, tag = "8")]
    pub ending_time_in_second_accuracy: i64,
    #[prost(string, repeated, tag = "9")]
    pub rewards: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "10")]
    pub questionnaire_url: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub data_center_questionnaire_id: ::prost::alloc::string::String,
    #[prost(string, tag = "12")]
    pub questionnaire_title: ::prost::alloc::string::String,
    #[prost(string, tag = "13")]
    pub group_id: ::prost::alloc::string::String,
    #[prost(string, tag = "21")]
    pub rewards_mail_title: ::prost::alloc::string::String,
    #[prost(string, tag = "22")]
    pub rewards_mail_text_content: ::prost::alloc::string::String,
    #[prost(bool, tag = "23")]
    pub online: bool,
    #[prost(sint64, tag = "24")]
    pub index: i64,
    #[prost(string, tag = "25")]
    pub ja_title: ::prost::alloc::string::String,
    #[prost(string, tag = "26")]
    pub ja_content: ::prost::alloc::string::String,
    #[prost(string, tag = "27")]
    pub zh_tw_title: ::prost::alloc::string::String,
    #[prost(string, tag = "28")]
    pub zh_tw_content: ::prost::alloc::string::String,
    #[prost(string, tag = "29")]
    pub en_title: ::prost::alloc::string::String,
    #[prost(string, tag = "30")]
    pub en_content: ::prost::alloc::string::String,
    #[prost(string, tag = "31")]
    pub ko_title: ::prost::alloc::string::String,
    #[prost(string, tag = "32")]
    pub ko_content: ::prost::alloc::string::String,
    #[prost(string, tag = "33")]
    pub th_title: ::prost::alloc::string::String,
    #[prost(string, tag = "34")]
    pub th_content: ::prost::alloc::string::String,
    #[prost(string, tag = "40")]
    pub questionnaire_ja_title: ::prost::alloc::string::String,
    #[prost(string, tag = "41")]
    pub questionnaire_zh_tw_title: ::prost::alloc::string::String,
    #[prost(string, tag = "42")]
    pub questionnaire_en_title: ::prost::alloc::string::String,
    #[prost(string, tag = "43")]
    pub questionnaire_ko_title: ::prost::alloc::string::String,
    #[prost(string, tag = "44")]
    pub questionnaire_th_title: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "50")]
    pub player_constrain_type_values: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    #[prost(sint64, tag = "91")]
    pub created_at: i64,
    #[prost(sint64, tag = "92")]
    pub updated_at: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PlayerConstrainType {
    InvalidConstrainType = 0,
    ClearChapterId = 1,
    PlayerLevelEqualTo = 2,
    PlayerLevelGreaterThanOrEqualTo = 3,
    LoginTimeEqualToInDay = 4,
    LoginTimeGreaterThanOrEqualTo = 5,
    NoConstraint = 6,
}
impl PlayerConstrainType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PlayerConstrainType::InvalidConstrainType => "INVALID_CONSTRAIN_TYPE",
            PlayerConstrainType::ClearChapterId => "CLEAR_CHAPTER_ID",
            PlayerConstrainType::PlayerLevelEqualTo => "PLAYER_LEVEL_EQUAL_TO",
            PlayerConstrainType::PlayerLevelGreaterThanOrEqualTo => {
                "PLAYER_LEVEL_GREATER_THAN_OR_EQUAL_TO"
            }
            PlayerConstrainType::LoginTimeEqualToInDay => "LOGIN_TIME_EQUAL_TO_IN_DAY",
            PlayerConstrainType::LoginTimeGreaterThanOrEqualTo => {
                "LOGIN_TIME_GREATER_THAN_OR_EQUAL_TO"
            }
            PlayerConstrainType::NoConstraint => "NO_CONSTRAINT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INVALID_CONSTRAIN_TYPE" => Some(Self::InvalidConstrainType),
            "CLEAR_CHAPTER_ID" => Some(Self::ClearChapterId),
            "PLAYER_LEVEL_EQUAL_TO" => Some(Self::PlayerLevelEqualTo),
            "PLAYER_LEVEL_GREATER_THAN_OR_EQUAL_TO" => {
                Some(Self::PlayerLevelGreaterThanOrEqualTo)
            }
            "LOGIN_TIME_EQUAL_TO_IN_DAY" => Some(Self::LoginTimeEqualToInDay),
            "LOGIN_TIME_GREATER_THAN_OR_EQUAL_TO" => {
                Some(Self::LoginTimeGreaterThanOrEqualTo)
            }
            "NO_CONSTRAINT" => Some(Self::NoConstraint),
            _ => None,
        }
    }
}
