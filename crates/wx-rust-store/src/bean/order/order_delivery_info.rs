//! 对应 Java `com.binarywang.wxjava.store.bean.order.OrderDeliveryInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::{DeliveryProductInfo, OrderAddressInfo, QualityInsepctInfo, RechargeInfo};

/// 微信小店 OrderDeliveryInfo 数据类型；对应 Java com.binarywang.wxjava.store.bean.order.OrderDeliveryInfo.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OrderDeliveryInfo {
    #[serde(rename = "address_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_info: Option<OrderAddressInfo>,
    #[serde(rename = "delivery_product_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_product_infos: Option<Vec<DeliveryProductInfo>>,
    #[serde(rename = "ship_done_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ship_done_time: Option<i64>,
    #[serde(rename = "deliver_method", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deliver_method: Option<i32>,
    #[serde(rename = "address_under_review", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_under_review: Option<OrderAddressInfo>,
    #[serde(rename = "address_apply_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_apply_time: Option<i64>,
    #[serde(rename = "ewaybill_order_code", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ewaybill_order_code: Option<String>,
    #[serde(rename = "quality_inspect_type", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_inspect_type: Option<String>,
    #[serde(rename = "quality_inspect_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_inspect_info: Option<QualityInsepctInfo>,
    #[serde(rename = "recharge_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recharge_info: Option<RechargeInfo>,
}
