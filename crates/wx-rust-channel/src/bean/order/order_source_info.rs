//! 对应 Java `me.chanjar.weixin.channel.bean.order.OrderSourceInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OrderSourceInfo {
    #[serde(rename = "sku_id", default)]
    pub sku_id: String,
    #[serde(rename = "account_type", default)]
    pub account_type: i32,
    #[serde(rename = "account_id", default)]
    pub account_id: String,
    #[serde(rename = "sale_channel", default)]
    pub sale_channel: i32,
    #[serde(rename = "account_nickname", default)]
    pub account_nickname: String,
    #[serde(rename = "content_type", default)]
    pub content_type: String,
    #[serde(rename = "content_id", default)]
    pub content_id: String,
    #[serde(rename = "promoter_head_supplier_id", default)]
    pub promoter_head_supplier_id: String,
}
