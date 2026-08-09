//! 对应 Java `me.chanjar.weixin.channel.bean.coupon.UserCoupon.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UserCoupon {
    #[serde(rename = "coupon_id", default)]
    pub coupon_id: String,
    #[serde(rename = "user_coupon_id", default)]
    pub user_coupon_id: String,
    #[serde(rename = "status", default)]
    pub status: i32,
    #[serde(rename = "create_time", default)]
    pub create_time: i64,
    #[serde(rename = "update_time", default)]
    pub update_time: i64,
    #[serde(rename = "start_time", default)]
    pub start_time: i64,
    #[serde(rename = "end_time", default)]
    pub end_time: i64,
    #[serde(rename = "ext_info", default)]
    pub ext_info: UserExtInfo,
    #[serde(rename = "order_id", default)]
    pub order_id: String,
    #[serde(rename = "discount_fee", default)]
    pub discount_fee: i32,
}
