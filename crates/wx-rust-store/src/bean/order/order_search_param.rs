//! 对应 Java `com.binarywang.wxjava.store.bean.order.OrderSearchParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::OrderSearchCondition;

/// 微信小店 OrderSearchParam 数据类型；对应 Java com.binarywang.wxjava.store.bean.order.OrderSearchParam.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OrderSearchParam {
    #[serde(rename = "page_size", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(rename = "next_key", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_key: Option<String>,
    #[serde(rename = "search_condition", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_condition: Option<OrderSearchCondition>,
    #[serde(rename = "on_aftersale_order_exist", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_after_sale_order_exist: Option<i32>,
    #[serde(rename = "status", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}
