//! 对应 Java `com.binarywang.wxjava.store.bean.coupon.UserCoupon.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::UserExtInfo;

/// 微信小店 UserCoupon 数据类型；对应 Java com.binarywang.wxjava.store.bean.coupon.UserCoupon.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UserCoupon {
    #[serde(rename = "coupon_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon_id: Option<String>,
    #[serde(rename = "user_coupon_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_coupon_id: Option<String>,
    #[serde(rename = "status", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "create_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    #[serde(rename = "update_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<i64>,
    #[serde(rename = "start_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    #[serde(rename = "end_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    #[serde(rename = "ext_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_info: Option<UserExtInfo>,
    #[serde(rename = "order_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(rename = "discount_fee", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_fee: Option<i32>,
}
