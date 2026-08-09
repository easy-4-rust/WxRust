//! 对应 Java `me.chanjar.weixin.open.bean.minishop.coupon.WxMinishopCouponStock.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::minishop::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMinishopCouponStock {
    #[serde(rename = "couponId", default)]
    pub coupon_id: i32,
    #[serde(rename = "type", default)]
    pub r#type: i32,
    #[serde(rename = "status", default)]
    pub status: i32,
    #[serde(rename = "createTime", default)]
    pub create_time: String,
    #[serde(rename = "updateTime", default)]
    pub update_time: String,
    #[serde(rename = "couponInfo", default)]
    pub coupon_info: WxMinishopCoupon,
    #[serde(rename = "stockInfo", default)]
    pub stock_info: WxMinishopCouponStockInfo,
}
