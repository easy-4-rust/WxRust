//! 优惠券信息（会员积分兑换）。
//!
//! 对应 Java `com.binarywang.wxjava.store.bean.message.vip.CouponInfo.java`。

use serde::{Deserialize, Serialize};

/// 优惠券信息（对应 Java `CouponInfo`）。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CouponInfo {
    /// 兑换的优惠券ID（对应 Java `relatedCouponId`）。
    #[serde(
        rename = "related_coupon_id",
        default,
        deserialize_with = "crate::bean::message::serde_helpers::opt_string_or_i64"
    )]
    pub related_coupon_id: Option<i64>,
}
