//! 对应 Java `com.binarywang.wxjava.store.bean.order.OrderPayInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 OrderPayInfo；对应 Java com.binarywang.wxjava.store.bean.order.OrderPayInfo.java。
pub struct OrderPayInfo {
    #[serde(rename = "payment_method", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<i32>,
    #[serde(rename = "pay_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_time: Option<i64>,
    #[serde(rename = "transaction_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
}
