//! 对应 Java `me.chanjar.weixin.channel.bean.brand.BasicBrand.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BasicBrand {
    #[serde(rename = "brand_id", default)]
    pub brand_id: String,
    #[serde(rename = "ch_name", default)]
    pub ch_name: String,
    #[serde(rename = "en_name", default)]
    pub en_name: String,
}
