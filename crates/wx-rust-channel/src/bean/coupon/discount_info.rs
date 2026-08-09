//! 对应 Java `me.chanjar.weixin.channel.bean.coupon.DiscountInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DiscountInfo {
    #[serde(rename = "discount_num", default)]
    pub discount_num: i32,
    #[serde(rename = "discount_fee", default)]
    pub discount_fee: i32,
    #[serde(rename = "discount_condition", default)]
    pub discount_condition: DiscountCondition,
}
