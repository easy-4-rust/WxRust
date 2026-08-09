//! 对应 Java `me.chanjar.weixin.channel.bean.product.SkuDeliverInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SkuDeliverInfo {
    #[serde(rename = "stock_type", default)]
    pub stock_type: i32,
    #[serde(rename = "full_payment_presale_delivery_type", default)]
    pub full_payment_presale_delivery_type: i32,
    #[serde(rename = "presale_begin_time", default)]
    pub presale_begin_time: i64,
    #[serde(rename = "presale_end_time", default)]
    pub presale_end_time: i64,
    #[serde(rename = "full_payment_presale_delivery_time", default)]
    pub full_payment_presale_delivery_time: i32,
}
