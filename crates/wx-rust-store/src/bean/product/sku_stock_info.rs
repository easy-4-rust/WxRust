//! 对应 Java `com.binarywang.wxjava.store.bean.product.SkuStockInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::WarehouseStockInfo;

/// 微信小店 SkuStockInfo 数据类型；对应 Java com.binarywang.wxjava.store.bean.product.SkuStockInfo.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SkuStockInfo {
    #[serde(rename = "normal_stock_num", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normal_stock_num: Option<i32>,
    #[serde(rename = "limited_discount_stock_num", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limited_discount_stock_num: Option<i32>,
    #[serde(rename = "warehouse_stocks", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warehouse_stocks: Option<Vec<WarehouseStockInfo>>,
    #[serde(rename = "total_stock_num", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_stock_num: Option<i32>,
    #[serde(rename = "finder_stock_num", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finder_total_num: Option<i32>,
}
