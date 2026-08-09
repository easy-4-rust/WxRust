//! 优惠券操作信息。
//!
//! 对应 Java `me.chanjar.weixin.channel.bean.message.coupon.CouponActionInfo.java`。

use serde::{Deserialize, Serialize};

/// 优惠券操作信息（对应 Java `CouponActionInfo`）。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CouponActionInfo {
    /// 优惠券ID（对应 Java `couponId`）。
    #[serde(rename = "coupon_id", default)]
    pub coupon_id: Option<String>,
    /// 领券时间（对应 Java `createTime`）。
    #[serde(rename = "create_time", default)]
    pub create_time: Option<String>,
    /// 删除时间（对应 Java `deleteTime`）。
    #[serde(rename = "delete_time", default)]
    pub delete_time: Option<String>,
    /// 过期时间（对应 Java `expireTime`）。
    #[serde(rename = "expire_time", default)]
    pub expire_time: Option<String>,
    /// 更新时间（对应 Java `changeTime`）。
    #[serde(rename = "change_time", default)]
    pub change_time: Option<String>,
    /// 作废时间（对应 Java `invalidTime`）。
    #[serde(rename = "invalid_time", default)]
    pub invalid_time: Option<String>,
}
