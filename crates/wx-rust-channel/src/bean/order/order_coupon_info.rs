//! 对应 Java `me.chanjar.weixin.channel.bean.order.OrderCouponInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OrderCouponInfo {
    #[serde(rename = "user_coupon_id", default)]
    pub user_coupon_id: String,
    #[serde(rename = "coupon_type", default)]
    pub coupon_type: i32,
    #[serde(rename = "discounted_price", default)]
    pub discounted_price: i32,
    #[serde(rename = "coupon_id", default)]
    pub coupon_id: String,
}
