//! 对应 Java `com.binarywang.wxjava.store.bean.limit.LimitSku.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 LimitSku；对应 Java com.binarywang.wxjava.store.bean.limit.LimitSku.java。
pub struct LimitSku {
    #[serde(rename = "sku_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku_id: Option<String>,
    #[serde(rename = "sale_price", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sale_price: Option<i32>,
    #[serde(rename = "sale_stock", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sale_stock: Option<i32>,
}
