//! 对应 Java `me.chanjar.weixin.channel.bean.brand.BrandGrantDetail.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BrandGrantDetail {
    #[serde(rename = "grant_certifications", default)]
    pub grant_certifications: Vec<String>,
    #[serde(rename = "grant_level", default)]
    pub grant_level: i32,
    #[serde(rename = "start_time", default)]
    pub start_time: i64,
    #[serde(rename = "end_time", default)]
    pub end_time: i64,
    #[serde(rename = "is_permanent", default)]
    pub permanent: bool,
    #[serde(rename = "brand_owner_id_photos", default)]
    pub brand_owner_id_photos: Vec<String>,
}
