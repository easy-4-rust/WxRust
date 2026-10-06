//! 对应 Java `com.binarywang.wxjava.store.bean.coupon.UserCouponListResponse.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::UserCouponIdInfo;

/// 微信小店 UserCouponListResponse 数据类型；对应 Java com.binarywang.wxjava.store.bean.coupon.UserCouponListResponse.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UserCouponListResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "user_coupon_list", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupons: Option<Vec<UserCouponIdInfo>>,
    #[serde(rename = "total_num", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_num: Option<i32>,
    #[serde(rename = "page_ctx", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_ctx: Option<String>,
}
