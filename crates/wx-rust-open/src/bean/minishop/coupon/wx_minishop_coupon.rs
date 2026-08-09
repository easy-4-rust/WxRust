//! 对应 Java `me.chanjar.weixin.open.bean.minishop.coupon.WxMinishopCoupon.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::minishop::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMinishopCoupon {
    #[serde(rename = "couponId", default)]
    pub coupon_id: i32,
    #[serde(rename = "status", default)]
    pub status: i32,
    #[serde(rename = "type", default)]
    pub r#type: i32,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "discountInfo", default)]
    pub discount_info: WxMinishopCouponDiscountInfo,
    #[serde(rename = "extInfo", default)]
    pub ext_info: WxMinishopCouponExtInfo,
    #[serde(rename = "promoteInfo", default)]
    pub promote_info: WxMinishopCouponPromoteInfo,
    #[serde(rename = "receiveInfo", default)]
    pub receive_info: WxMinishopCouponReceiveInfo,
    #[serde(rename = "validInfo", default)]
    pub valid_info: WxMinishopCouponValidInfo,
}
