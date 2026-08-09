//! 对应 Java `me.chanjar.weixin.open.bean.minishop.coupon.WxMinishopCouponPromoteInfo.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::minishop::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMinishopCouponPromoteInfo {
    #[serde(rename = "customizeChannel", default)]
    pub customize_channel: String,
    #[serde(rename = "promotionType", default)]
    pub promotion_type: i32,
}
