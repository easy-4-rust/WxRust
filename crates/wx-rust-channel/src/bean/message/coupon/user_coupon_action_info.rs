//! 用户优惠券操作信息。
//!
//! 对应 Java `me.chanjar.weixin.channel.bean.message.coupon.UserCouponActionInfo.java`。

use serde::{Deserialize, Serialize};

/// 用户优惠券操作信息（对应 Java `UserCouponActionInfo`）。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct UserCouponActionInfo {
    /// 优惠券ID（对应 Java `couponId`）。
    #[serde(rename = "coupon_id", default)]
    pub coupon_id: Option<String>,
    /// 用户券ID（对应 Java `userCouponId`）。
    #[serde(rename = "user_coupon_id", default)]
    pub user_coupon_id: Option<String>,
    /// 过期时间（对应 Java `expireTime`）。
    #[serde(rename = "expire_time", default)]
    pub expire_time: Option<String>,
    /// 使用时间（对应 Java `useTime`）。
    #[serde(rename = "use_time", default)]
    pub use_time: Option<String>,
    /// 返还时间（对应 Java `unuseTime`）。
    #[serde(rename = "unuse_time", default)]
    pub unuse_time: Option<String>,
}
