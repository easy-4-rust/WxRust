//! 对应 Java `me.chanjar.weixin.channel.bean.order.OrderPayInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OrderPayInfo {
    #[serde(rename = "payment_method", default)]
    pub payment_method: i32,
    #[serde(rename = "pay_time", default)]
    pub pay_time: i64,
    #[serde(rename = "transaction_id", default)]
    pub transaction_id: String,
}
