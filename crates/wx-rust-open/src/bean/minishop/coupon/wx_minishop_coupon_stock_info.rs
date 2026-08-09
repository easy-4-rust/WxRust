//! 对应 Java `me.chanjar.weixin.open.bean.minishop.coupon.WxMinishopCouponStockInfo.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::minishop::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMinishopCouponStockInfo {
    #[serde(rename = "issuedNum", default)]
    pub issued_num: i32,
    #[serde(rename = "receiveNum", default)]
    pub receive_num: i32,
    #[serde(rename = "usedNum", default)]
    pub used_num: i32,
}
