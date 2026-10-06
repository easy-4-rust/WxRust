//! 对应 Java `com.binarywang.wxjava.store.bean.order.OrderSearchCondition.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 OrderSearchCondition；对应 Java com.binarywang.wxjava.store.bean.order.OrderSearchCondition.java。
pub struct OrderSearchCondition {
    #[serde(rename = "title", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "sku_code", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku_code: Option<String>,
    #[serde(rename = "user_name", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(rename = "tel_number", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tel_number: Option<String>,
    #[serde(rename = "tel_number_last4", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tel_number_last4: Option<String>,
    #[serde(rename = "order_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(rename = "merchant_notes", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_notes: Option<String>,
    #[serde(rename = "customer_notes", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_notes: Option<String>,
    #[serde(rename = "address_under_review", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_under_review: Option<bool>,
}
