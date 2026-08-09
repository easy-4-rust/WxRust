//! 对应 Java `me.chanjar.weixin.open.bean.minishop.coupon.WxMinishopCouponExtInfo.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::minishop::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMinishopCouponExtInfo {
    #[serde(rename = "notes", default)]
    pub notes: String,
    #[serde(rename = "validTime", default)]
    pub valid_time: i64,
    #[serde(rename = "invalidTime", default)]
    pub invalid_time: i64,
    #[serde(rename = "jumpProductId", default)]
    pub jump_product_id: i64,
}
