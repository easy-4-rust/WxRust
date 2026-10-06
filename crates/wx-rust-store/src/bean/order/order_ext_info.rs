//! 对应 Java `com.binarywang.wxjava.store.bean.order.OrderExtInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 OrderExtInfo；对应 Java com.binarywang.wxjava.store.bean.order.OrderExtInfo.java。
pub struct OrderExtInfo {
    #[serde(rename = "customer_notes", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_notes: Option<String>,
    #[serde(rename = "merchant_notes", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_notes: Option<String>,
    #[serde(rename = "confirm_receipt_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm_receipt_time: Option<i64>,
    #[serde(rename = "finder_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finder_id: Option<String>,
    #[serde(rename = "live_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_id: Option<String>,
    #[serde(rename = "order_scene", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_scene: Option<i32>,
}
