//! 对应 Java `me.chanjar.weixin.channel.bean.order.OrderDeliveryInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OrderDeliveryInfo {
    #[serde(rename = "address_info", default)]
    pub address_info: OrderAddressInfo,
    #[serde(rename = "delivery_product_info", default)]
    pub delivery_product_infos: Vec<DeliveryProductInfo>,
    #[serde(rename = "ship_done_time", default)]
    pub ship_done_time: i64,
    #[serde(rename = "deliver_method", default)]
    pub deliver_method: i32,
    #[serde(rename = "address_under_review", default)]
    pub address_under_review: OrderAddressInfo,
    #[serde(rename = "address_apply_time", default)]
    pub address_apply_time: i64,
    #[serde(rename = "ewaybill_order_code", default)]
    pub ewaybill_order_code: String,
    #[serde(rename = "quality_inspect_type", default)]
    pub quality_inspect_type: String,
    #[serde(rename = "quality_inspect_info", default)]
    pub quality_inspect_info: QualityInsepctInfo,
    #[serde(rename = "recharge_info", default)]
    pub recharge_info: RechargeInfo,
}
