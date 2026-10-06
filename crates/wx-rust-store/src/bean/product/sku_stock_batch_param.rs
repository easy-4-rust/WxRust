//! 对应 Java `com.binarywang.wxjava.store.bean.product.SkuStockBatchParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 SkuStockBatchParam；对应 Java com.binarywang.wxjava.store.bean.product.SkuStockBatchParam.java。
pub struct SkuStockBatchParam {
    #[serde(rename = "product_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_ids: Option<Vec<String>>,
}
