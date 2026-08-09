//! 对应 Java `me.chanjar.weixin.channel.bean.product.SkuStockInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SkuStockInfo {
    #[serde(rename = "normal_stock_num", default)]
    pub normal_stock_num: i32,
    #[serde(rename = "limited_discount_stock_num", default)]
    pub limited_discount_stock_num: i32,
    #[serde(rename = "warehouse_stocks", default)]
    pub warehouse_stocks: Vec<WarehouseStockInfo>,
    #[serde(rename = "total_stock_num", default)]
    pub total_stock_num: i32,
    #[serde(rename = "finder_stock_num", default)]
    pub finder_total_num: i32,
}
