//! 对应 Java `me.chanjar.weixin.channel.bean.coupon.UserCouponListResponse.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[allow(unused_imports)]
use crate::bean::base::WxChannelBaseResponse;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UserCouponListResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "user_coupon_list", default)]
    pub coupons: Vec<UserCouponIdInfo>,
    #[serde(rename = "total_num", default)]
    pub total_num: i32,
    #[serde(rename = "page_ctx", default)]
    pub page_ctx: String,
}
