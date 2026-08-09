//! 对应 Java `me.chanjar.weixin.channel.bean.product.SpuStockInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SpuStockInfo {
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    #[serde(rename = "sku_stock", default)]
    pub sku_stock: Vec<SkuStockInfo>,
}
