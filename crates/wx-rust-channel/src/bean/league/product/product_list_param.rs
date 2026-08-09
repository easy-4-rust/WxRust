//! 对应 Java `me.chanjar.weixin.channel.bean.league.product.ProductListParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::league::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProductListParam {
    #[serde(rename = "type", default)]
    pub r#type: i32,
    #[serde(rename = "page_size", default)]
    pub page_size: i32,
    #[serde(rename = "page_index", default)]
    pub page_index: i32,
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    #[serde(rename = "finder_id", default)]
    pub finder_id: String,
    #[serde(rename = "last_buffer", default)]
    pub last_buffer: String,
    #[serde(rename = "need_total_num", default)]
    pub need_total_num: bool,
}
