//! 对应 Java `com.binarywang.wxjava.store.bean.product.SkuStockParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 SkuStockParam；对应 Java com.binarywang.wxjava.store.bean.product.SkuStockParam.java。
pub struct SkuStockParam {
    #[serde(rename = "product_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(rename = "sku_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku_id: Option<String>,
    #[serde(rename = "diff_type", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diff_type: Option<i32>,
    #[serde(rename = "num", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num: Option<i32>,
}
