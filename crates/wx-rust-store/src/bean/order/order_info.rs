//! 对应 Java `com.binarywang.wxjava.store.bean.order.OrderInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::{AfterSaleDetail, OrderDetailInfo};

/// 微信小店 OrderInfo 数据类型；对应 Java com.binarywang.wxjava.store.bean.order.OrderInfo.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OrderInfo {
    #[serde(rename = "order_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(rename = "status", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "openid", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openid: Option<String>,
    #[serde(rename = "unionid", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unionid: Option<String>,
    #[serde(rename = "order_detail", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_detail: Option<OrderDetailInfo>,
    #[serde(rename = "aftersale_detail", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_sale_detail: Option<AfterSaleDetail>,
    #[serde(rename = "is_present", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub present: Option<bool>,
    #[serde(rename = "present_order_id_str", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub present_order_id: Option<String>,
    #[serde(rename = "present_note", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub present_note: Option<String>,
    #[serde(rename = "present_giver_openid", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub present_giver_openid: Option<String>,
    #[serde(rename = "present_giver_unionid", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub present_giver_unionid: Option<String>,
    #[serde(rename = "create_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i32>,
    #[serde(rename = "update_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<i32>,
}
