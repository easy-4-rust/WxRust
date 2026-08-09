//! 对应 Java `me.chanjar.weixin.channel.bean.order.OrderSearchCondition.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OrderSearchCondition {
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "sku_code", default)]
    pub sku_code: String,
    #[serde(rename = "user_name", default)]
    pub user_name: String,
    #[serde(rename = "tel_number", default)]
    pub tel_number: String,
    #[serde(rename = "tel_number_last4", default)]
    pub tel_number_last4: String,
    #[serde(rename = "order_id", default)]
    pub order_id: String,
    #[serde(rename = "merchant_notes", default)]
    pub merchant_notes: String,
    #[serde(rename = "customer_notes", default)]
    pub customer_notes: String,
    #[serde(rename = "address_under_review", default)]
    pub address_under_review: bool,
}
