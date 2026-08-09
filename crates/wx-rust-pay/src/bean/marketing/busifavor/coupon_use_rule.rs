//! 对应 Java `com.github.binarywang.wxpay.bean.marketing.busifavor.CouponUseRule.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CouponUseRule {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "coupon_available_time"
    )]
    pub coupon_available_time: Option<CouponAvailableTime>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "fixed_normal_coupon"
    )]
    pub fixed_normal_coupon: Option<FixedNormalCoupon>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "discount_coupon"
    )]
    pub discount_coupon: Option<DiscountCoupon>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "exchange_coupon"
    )]
    pub exchange_coupon: Option<ExchangeCoupon>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "use_method"
    )]
    pub use_method: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "mini_programs_appid"
    )]
    pub mini_programs_appid: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "mini_programs_path"
    )]
    pub mini_programs_path: Option<String>,
}
