//! 对应 Java `com.binarywang.wxjava.store.bean.coupon.CouponStatusParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 CouponStatusParam；对应 Java com.binarywang.wxjava.store.bean.coupon.CouponStatusParam.java。
pub struct CouponStatusParam {
    #[serde(rename = "coupon_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon_id: Option<String>,
    #[serde(rename = "status", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}
