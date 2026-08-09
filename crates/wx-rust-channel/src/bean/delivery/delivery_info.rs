//! 对应 Java `me.chanjar.weixin.channel.bean.delivery.DeliveryInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DeliveryInfo {
    #[serde(rename = "waybill_id", default)]
    pub waybill_id: String,
    #[serde(rename = "delivery_id", default)]
    pub delivery_id: String,
    #[serde(rename = "deliver_type", default)]
    pub deliver_type: i32,
    #[serde(rename = "product_infos", default)]
    pub product_infos: Vec<FreightProductInfo>,
}
