//! 对应 Java `com.binarywang.wxjava.store.bean.order.OrderInfoParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 OrderInfoParam；对应 Java com.binarywang.wxjava.store.bean.order.OrderInfoParam.java。
pub struct OrderInfoParam {
    #[serde(rename = "order_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(rename = "encode_sensitive_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encode_sensitive_info: Option<bool>,
}
