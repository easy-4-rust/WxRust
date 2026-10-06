//! 对应 Java `com.binarywang.wxjava.store.bean.coupon.CouponInfoResponse.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::CouponInfo;

/// 微信小店 CouponInfoResponse 数据类型；对应 Java com.binarywang.wxjava.store.bean.coupon.CouponInfoResponse.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CouponInfoResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "coupon", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<CouponInfo>,
}
