//! 对应 Java `com.binarywang.wxjava.store.bean.product.SpuSimpleInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::SkuInfo;

/// 微信小店 SpuSimpleInfo 数据类型；对应 Java com.binarywang.wxjava.store.bean.product.SpuSimpleInfo.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SpuSimpleInfo {
    #[serde(rename = "product_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(rename = "out_product_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_product_id: Option<String>,
    #[serde(rename = "skus", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skus: Option<Vec<SkuInfo>>,
}
