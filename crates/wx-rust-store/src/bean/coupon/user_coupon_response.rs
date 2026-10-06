//! 对应 Java `com.binarywang.wxjava.store.bean.coupon.UserCouponResponse.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::UserCoupon;

/// 微信小店 UserCouponResponse 数据类型；对应 Java com.binarywang.wxjava.store.bean.coupon.UserCouponResponse.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UserCouponResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "user_coupon", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<UserCoupon>,
    #[serde(rename = "openid", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openid: Option<String>,
    #[serde(rename = "unionid", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unionid: Option<String>,
}
