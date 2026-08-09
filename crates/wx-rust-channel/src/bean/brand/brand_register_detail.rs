//! 对应 Java `me.chanjar.weixin.channel.bean.brand.BrandRegisterDetail.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BrandRegisterDetail {
    #[serde(rename = "registrant", default)]
    pub registrant: String,
    #[serde(rename = "register_no", default)]
    pub register_no: String,
    #[serde(rename = "start_time", default)]
    pub start_time: i64,
    #[serde(rename = "end_time", default)]
    pub end_time: i64,
    #[serde(rename = "is_permanent", default)]
    pub permanent: bool,
    #[serde(rename = "register_certifications", default)]
    pub register_certifications: Vec<String>,
    #[serde(rename = "renew_certifications", default)]
    pub renew_certifications: Vec<String>,
}
