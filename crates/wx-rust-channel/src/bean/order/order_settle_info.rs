//! 对应 Java `me.chanjar.weixin.channel.bean.order.OrderSettleInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OrderSettleInfo {
    #[serde(rename = "predict_commission_fee", default)]
    pub predict_commission_fee: i32,
    #[serde(rename = "commission_fee", default)]
    pub commission_fee: i32,
    #[serde(rename = "predict_wecoin_commission", default)]
    pub predict_wecoin_commission: i32,
    #[serde(rename = "wecoin_commission", default)]
    pub wecoin_commission: i32,
    #[serde(rename = "settle_time", default)]
    pub settle_time: i64,
}
