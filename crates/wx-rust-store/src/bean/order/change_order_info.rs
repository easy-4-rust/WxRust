//! 对应 Java `com.binarywang.wxjava.store.bean.order.ChangeOrderInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 ChangeOrderInfo；对应 Java com.binarywang.wxjava.store.bean.order.ChangeOrderInfo.java。
pub struct ChangeOrderInfo {
    #[serde(rename = "product_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(rename = "sku_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku_id: Option<String>,
    #[serde(rename = "change_price", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_price: Option<String>,
}
