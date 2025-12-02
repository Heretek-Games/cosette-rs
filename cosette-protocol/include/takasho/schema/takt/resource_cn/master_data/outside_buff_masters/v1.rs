#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OutsideBuffMaster {
    #[prost(string, tag = "1")]
    pub master_version: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub outside_buff_master_id: i64,
    #[prost(enumeration = "OutsideBuffBenefitType", tag = "3")]
    pub benefit_type: i32,
    #[prost(int64, repeated, tag = "4")]
    pub job_limint: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, repeated, tag = "5")]
    pub tag_limit: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, repeated, tag = "6")]
    pub character_limit: ::prost::alloc::vec::Vec<i64>,
    #[prost(int64, tag = "7")]
    pub hp: i64,
    #[prost(int64, tag = "8")]
    pub hp_percent: i64,
    #[prost(int64, tag = "9")]
    pub atk: i64,
    #[prost(int64, tag = "10")]
    pub atk_percent: i64,
    #[prost(int64, tag = "11")]
    pub def: i64,
    #[prost(int64, tag = "12")]
    pub def_percent: i64,
    #[prost(int64, tag = "13")]
    pub int_atk: i64,
    #[prost(int64, tag = "14")]
    pub int_percent: i64,
    #[prost(int64, tag = "15")]
    pub res: i64,
    #[prost(int64, tag = "16")]
    pub res_percent: i64,
    #[prost(int64, tag = "17")]
    pub dex: i64,
    #[prost(int64, tag = "18")]
    pub dex_percent: i64,
    #[prost(int64, tag = "19")]
    pub crt: i64,
    #[prost(int64, tag = "20")]
    pub crt_percent: i64,
    #[prost(int64, tag = "21")]
    pub crd: i64,
    #[prost(int64, tag = "22")]
    pub crd_percent: i64,
    #[prost(int64, tag = "23")]
    pub acrt: i64,
    #[prost(int64, tag = "24")]
    pub acrt_percent: i64,
    #[prost(int64, tag = "25")]
    pub crdr: i64,
    #[prost(int64, tag = "26")]
    pub crdr_percnet: i64,
    #[prost(int64, tag = "27")]
    pub ap: i64,
    #[prost(int64, tag = "28")]
    pub ap_percent: i64,
    #[prost(int64, tag = "29")]
    pub buff_id: i64,
    #[prost(int64, tag = "30")]
    pub attach_type: i64,
    #[prost(int64, tag = "31")]
    pub attach_type_param1: i64,
    #[prost(int64, tag = "32")]
    pub attach_type_param2: i64,
    #[prost(int64, tag = "33")]
    pub attach_type_param3: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OutsideBuffMasters {
    #[prost(message, repeated, tag = "1")]
    pub outside_buff_masters: ::prost::alloc::vec::Vec<OutsideBuffMaster>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OutsideBuffBenefitType {
    OutsideBuffBenefitAllType = 0,
    OutsideBuffBenefitAttrType = 1,
    OutsideBuffBenefitBuffType = 2,
}
impl OutsideBuffBenefitType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OutsideBuffBenefitType::OutsideBuffBenefitAllType => {
                "OutsideBuffBenefitAllType"
            }
            OutsideBuffBenefitType::OutsideBuffBenefitAttrType => {
                "OutsideBuffBenefitAttrType"
            }
            OutsideBuffBenefitType::OutsideBuffBenefitBuffType => {
                "OutsideBuffBenefitBuffType"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OutsideBuffBenefitAllType" => Some(Self::OutsideBuffBenefitAllType),
            "OutsideBuffBenefitAttrType" => Some(Self::OutsideBuffBenefitAttrType),
            "OutsideBuffBenefitBuffType" => Some(Self::OutsideBuffBenefitBuffType),
            _ => None,
        }
    }
}
