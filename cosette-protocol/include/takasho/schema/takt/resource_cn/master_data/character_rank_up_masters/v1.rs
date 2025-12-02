#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CharacterRankUpMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub character_rank_up_master_id: i64,
    #[prost(int64, tag = "3")]
    pub battle_character_master_id: i64,
    #[prost(enumeration = "Rank", tag = "11")]
    pub rank: i32,
    #[prost(string, repeated, tag = "12")]
    pub require_contents: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, tag = "21")]
    pub hp_growth_value: i64,
    #[prost(int64, tag = "22")]
    pub atk_growth_value: i64,
    #[prost(int64, tag = "23")]
    pub def_growth_value: i64,
    #[prost(int64, tag = "24")]
    pub dex_growth_value: i64,
    #[prost(int64, tag = "25")]
    pub int_growth_value: i64,
    #[prost(int64, tag = "26")]
    pub res_growth_value: i64,
    #[prost(int64, tag = "27")]
    pub crt_growth_value: i64,
    #[prost(int64, tag = "28")]
    pub crd_growth_value: i64,
    #[prost(int64, tag = "29")]
    pub acrt_growth_value: i64,
    #[prost(int64, tag = "30")]
    pub crdr_growth_value: i64,
    #[prost(int64, tag = "31")]
    pub musical_effect_id: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CharacterRankUpMasters {
    #[prost(message, repeated, tag = "1")]
    pub character_rank_up_masters: ::prost::alloc::vec::Vec<CharacterRankUpMaster>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Rank {
    InvalidRank = 0,
    OneStar = 1,
    TwoStar = 2,
    ThreeStar = 3,
    FourStar = 4,
    FiveStar = 5,
}
impl Rank {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Rank::InvalidRank => "INVALID_RANK",
            Rank::OneStar => "ONE_STAR",
            Rank::TwoStar => "TWO_STAR",
            Rank::ThreeStar => "THREE_STAR",
            Rank::FourStar => "FOUR_STAR",
            Rank::FiveStar => "FIVE_STAR",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INVALID_RANK" => Some(Self::InvalidRank),
            "ONE_STAR" => Some(Self::OneStar),
            "TWO_STAR" => Some(Self::TwoStar),
            "THREE_STAR" => Some(Self::ThreeStar),
            "FOUR_STAR" => Some(Self::FourStar),
            "FIVE_STAR" => Some(Self::FiveStar),
            _ => None,
        }
    }
}
