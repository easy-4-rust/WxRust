//! 对应 Java `me.chanjar.weixin.open.bean.minishop.coupon.WxMinishopCouponReceiveInfo.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::minishop::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMinishopCouponReceiveInfo {
    #[serde(rename = "endTime", default)]
    pub end_time: i64,
    #[serde(rename = "limitNumOnePerson", default)]
    pub limit_num_one_person: i32,
    #[serde(rename = "startTime", default)]
    pub start_time: i64,
    #[serde(rename = "totalNum", default)]
    pub total_num: i32,
}
