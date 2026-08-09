//! 对应 Java `me.chanjar.weixin.channel.bean.order.OrderExtInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OrderExtInfo {
    #[serde(rename = "customer_notes", default)]
    pub customer_notes: String,
    #[serde(rename = "merchant_notes", default)]
    pub merchant_notes: String,
    #[serde(rename = "confirm_receipt_time", default)]
    pub confirm_receipt_time: i64,
    #[serde(rename = "finder_id", default)]
    pub finder_id: String,
    #[serde(rename = "live_id", default)]
    pub live_id: String,
    #[serde(rename = "order_scene", default)]
    pub order_scene: i32,
}
