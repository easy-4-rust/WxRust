//! 对应 Java `com.binarywang.wxjava.store.bean.coupon.UserCouponListParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 UserCouponListParam；对应 Java com.binarywang.wxjava.store.bean.coupon.UserCouponListParam.java。
pub struct UserCouponListParam {
    #[serde(rename = "status", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "page", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(rename = "page_size", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(rename = "page_ctx", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_ctx: Option<String>,
    #[serde(rename = "openid", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
}
