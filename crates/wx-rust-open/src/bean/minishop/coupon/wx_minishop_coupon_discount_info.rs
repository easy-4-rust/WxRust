//! 对应 Java `me.chanjar.weixin.open.bean.minishop.coupon.WxMinishopCouponDiscountInfo.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::minishop::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMinishopCouponDiscountInfo {
    #[serde(rename = "discountCondition", default)]
    pub discount_condition: WxMinishopCouponDiscountCondition,
    #[serde(rename = "discountFee", default)]
    pub discount_fee: i32,
    #[serde(rename = "discountNum", default)]
    pub discount_num: i32,
}
