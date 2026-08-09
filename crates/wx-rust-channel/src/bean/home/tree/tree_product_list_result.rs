//! 对应 Java `me.chanjar.weixin.channel.bean.home.tree.TreeProductListResult.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::home::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TreeProductListResult {
    #[serde(rename = "product_ids", default)]
    pub product_ids: Vec<i64>,
    #[serde(rename = "total_count", default)]
    pub total_count: i32,
    #[serde(rename = "page_context", default)]
    pub page_context: String,
}
