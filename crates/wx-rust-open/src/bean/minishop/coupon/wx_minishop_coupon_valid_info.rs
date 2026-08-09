//! 对应 Java `me.chanjar.weixin.open.bean.minishop.coupon.WxMinishopCouponValidInfo.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::minishop::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMinishopCouponValidInfo {
    #[serde(rename = "endTime", default)]
    pub end_time: i64,
    #[serde(rename = "startTime", default)]
    pub start_time: i64,
    #[serde(rename = "validDayNum", default)]
    pub valid_day_num: i32,
    #[serde(rename = "validType", default)]
    pub valid_type: i32,
}
