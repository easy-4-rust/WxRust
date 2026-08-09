//! 对应 Java `me.chanjar.weixin.open.bean.minishop.coupon.WxMinishopCouponDiscountCondition.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::minishop::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMinishopCouponDiscountCondition {
    #[serde(rename = "productCnt", default)]
    pub product_cnt: i32,
    #[serde(rename = "productIds", default)]
    pub product_ids: Vec<i32>,
    #[serde(rename = "productPrice", default)]
    pub product_price: i32,
}
