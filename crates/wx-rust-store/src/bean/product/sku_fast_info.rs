//! 对应 Java `com.binarywang.wxjava.store.bean.product.SkuFastInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::SkuDeliverInfo;

/// 微信小店 SkuFastInfo 数据类型；对应 Java com.binarywang.wxjava.store.bean.product.SkuFastInfo.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SkuFastInfo {
    #[serde(rename = "sku_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku_id: Option<String>,
    #[serde(rename = "sale_price", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sale_price: Option<i32>,
    #[serde(rename = "stock_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stock_info: Option<StockInfo>,
    #[serde(rename = "sku_deliver_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku_deliver_info: Option<SkuDeliverInfo>,
    #[serde(rename = "is_delete", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<bool>,
    #[serde(rename = "sku_code", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku_code: Option<String>,
    #[serde(rename = "status", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

/// 微信小店 StockInfo 数据类型；对应 Java com.binarywang.wxjava.store.bean.product.SkuFastInfo.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StockInfo {
    #[serde(rename = "diff_type", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diff_type: Option<i32>,
    #[serde(rename = "num", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num: Option<i32>,
}
