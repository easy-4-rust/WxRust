//! 对应 Java `me.chanjar.weixin.channel.bean.coupon.UserCouponListParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UserCouponListParam {
    #[serde(rename = "status", default)]
    pub status: i32,
    #[serde(rename = "page", default)]
    pub page: i32,
    #[serde(rename = "page_size", default)]
    pub page_size: i32,
    #[serde(rename = "page_ctx", default)]
    pub page_ctx: String,
    #[serde(rename = "openid", default)]
    pub open_id: String,
}
