//! 对应 Java `me.chanjar.weixin.channel.bean.order.OrderSkuDeliverInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OrderSkuDeliverInfo {
    #[serde(rename = "stock_type", default)]
    pub stock_type: i32,
    #[serde(rename = "predict_delivery_time", default)]
    pub predict_delivery_time: String,
}
