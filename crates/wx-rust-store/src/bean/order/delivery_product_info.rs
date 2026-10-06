//! 对应 Java `com.binarywang.wxjava.store.bean.order.DeliveryProductInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::OrderAddressInfo;

#[allow(unused_imports)]
use crate::bean::delivery::FreightProductInfo;

/// 微信小店 DeliveryProductInfo 数据类型；对应 Java com.binarywang.wxjava.store.bean.order.DeliveryProductInfo.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DeliveryProductInfo {
    #[serde(rename = "waybill_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waybill_id: Option<String>,
    #[serde(rename = "delivery_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_id: Option<String>,
    #[serde(rename = "product_infos", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_infos: Option<Vec<FreightProductInfo>>,
    #[serde(rename = "delivery_name", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_name: Option<String>,
    #[serde(rename = "delivery_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_time: Option<i64>,
    #[serde(rename = "deliver_type", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deliver_type: Option<i32>,
    #[serde(rename = "delivery_address", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_address: Option<OrderAddressInfo>,
}
