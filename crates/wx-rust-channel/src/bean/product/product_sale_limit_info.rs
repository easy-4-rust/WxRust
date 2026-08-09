//! 对应 Java `me.chanjar.weixin.channel.bean.product.ProductSaleLimitInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProductSaleLimitInfo {
    #[serde(rename = "is_limited", default)]
    pub limited: i32,
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "sub_title", default)]
    pub sub_title: String,
}
