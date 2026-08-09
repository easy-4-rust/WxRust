//! 对应 Java `me.chanjar.weixin.channel.bean.order.DeliveryProductInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[allow(unused_imports)]
use crate::bean::delivery::FreightProductInfo;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DeliveryProductInfo {
    #[serde(rename = "waybill_id", default)]
    pub waybill_id: String,
    #[serde(rename = "delivery_id", default)]
    pub delivery_id: String,
    #[serde(rename = "product_infos", default)]
    pub product_infos: Vec<FreightProductInfo>,
    #[serde(rename = "delivery_name", default)]
    pub delivery_name: String,
    #[serde(rename = "delivery_time", default)]
    pub delivery_time: i64,
    #[serde(rename = "deliver_type", default)]
    pub deliver_type: i32,
    #[serde(rename = "delivery_address", default)]
    pub delivery_address: OrderAddressInfo,
}
