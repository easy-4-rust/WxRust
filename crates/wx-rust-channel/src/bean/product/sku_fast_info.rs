//! 对应 Java `me.chanjar.weixin.channel.bean.product.SkuFastInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SkuFastInfo {
    #[serde(rename = "sku_id", default)]
    pub sku_id: String,
    #[serde(rename = "sale_price", default)]
    pub sale_price: i32,
    #[serde(rename = "stock_info", default)]
    pub stock_info: StockInfo,
    #[serde(rename = "sku_deliver_info", default)]
    pub sku_deliver_info: SkuDeliverInfo,
    #[serde(rename = "is_delete", default)]
    pub delete: bool,
    #[serde(rename = "sku_code", default)]
    pub sku_code: String,
    #[serde(rename = "status", default)]
    pub status: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StockInfo {
    #[serde(rename = "diff_type", default)]
    pub diff_type: i32,
    #[serde(rename = "num", default)]
    pub num: i32,
}
