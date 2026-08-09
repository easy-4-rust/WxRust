//! 积分兑换信息。
//!
//! 对应 Java `me.chanjar.weixin.channel.bean.message.vip.ExchangeInfo.java`。

use serde::{Deserialize, Serialize};

use crate::bean::message::vip::{CouponInfo, ProductInfo};

/// 积分兑换信息（对应 Java `ExchangeInfo`）。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ExchangeInfo {
    /// 入会时间（对应 Java `pay_score`）。
    #[serde(
        rename = "pay_score",
        default,
        deserialize_with = "crate::bean::message::serde_helpers::opt_string_or_i64"
    )]
    pub pay_score: Option<i64>,
    /// 兑换类型 1.优惠券 2商品（对应 Java `score_item_type`）。
    #[serde(
        rename = "score_item_type",
        default,
        deserialize_with = "crate::bean::message::serde_helpers::opt_string_or_i64"
    )]
    pub score_item_type: Option<i64>,
    /// 优惠券信息（对应 Java `couponInfo`）。
    #[serde(rename = "coupon_info", default)]
    pub coupon_info: Option<CouponInfo>,
    /// 商品信息（对应 Java `productInfo`）。
    #[serde(rename = "product_info", default)]
    pub product_info: Option<ProductInfo>,
}
