//! 对应 Java `com.binarywang.wxjava.store.bean.order.OrderGreetingCardInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 OrderGreetingCardInfo；对应 Java com.binarywang.wxjava.store.bean.order.OrderGreetingCardInfo.java。
pub struct OrderGreetingCardInfo {
    #[serde(rename = "giver_name", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giver_name: Option<String>,
    #[serde(rename = "receiver_name", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receiver_name: Option<String>,
    #[serde(rename = "greeting_message", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub greeting_message: Option<String>,
}
