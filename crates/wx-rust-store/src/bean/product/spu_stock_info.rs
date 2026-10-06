//! 对应 Java `com.binarywang.wxjava.store.bean.product.SpuStockInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::SkuStockInfo;

/// 微信小店 SpuStockInfo 数据类型；对应 Java com.binarywang.wxjava.store.bean.product.SpuStockInfo.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SpuStockInfo {
    #[serde(rename = "product_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(rename = "sku_stock", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku_stock: Option<Vec<SkuStockInfo>>,
}
