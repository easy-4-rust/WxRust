//! 对应 Java `com.binarywang.wxjava.store.bean.product.SkuStockBatchList.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::SpuStockInfo;

/// 微信小店 SkuStockBatchList 数据类型；对应 Java com.binarywang.wxjava.store.bean.product.SkuStockBatchList.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SkuStockBatchList {
    #[serde(rename = "spu_stock_list", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spu_stock_list: Option<Vec<SpuStockInfo>>,
}
