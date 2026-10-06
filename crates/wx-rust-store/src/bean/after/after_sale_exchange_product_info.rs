//! 对应 Java `com.binarywang.wxjava.store.bean.after.AfterSaleExchangeProductInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 AfterSaleExchangeProductInfo；对应 Java com.binarywang.wxjava.store.bean.after.AfterSaleExchangeProductInfo.java。
pub struct AfterSaleExchangeProductInfo {
    #[serde(rename = "product_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(rename = "old_sku_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_sku_id: Option<String>,
    #[serde(rename = "new_sku_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_sku_id: Option<String>,
    #[serde(rename = "product_cnt", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_cnt: Option<String>,
    #[serde(rename = "old_sku_price", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_sku_price: Option<i32>,
    #[serde(rename = "new_sku_price", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_sku_price: Option<i32>,
}
