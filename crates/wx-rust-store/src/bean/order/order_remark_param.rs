//! 对应 Java `com.binarywang.wxjava.store.bean.order.OrderRemarkParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 OrderRemarkParam；对应 Java com.binarywang.wxjava.store.bean.order.OrderRemarkParam.java。
pub struct OrderRemarkParam {
    #[serde(rename = "order_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(rename = "merchant_notes", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_notes: Option<String>,
}
