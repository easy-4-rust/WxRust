//! 对应 Java `com.binarywang.wxjava.store.bean.coupon.CouponInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::{CouponDetailInfo, StockInfo};

/// 微信小店 CouponInfo 数据类型；对应 Java com.binarywang.wxjava.store.bean.coupon.CouponInfo.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CouponInfo {
    #[serde(rename = "coupon_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon_id: Option<String>,
    #[serde(rename = "type", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
    #[serde(rename = "status", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "create_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    #[serde(rename = "update_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<i64>,
    #[serde(rename = "coupon_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<CouponDetailInfo>,
    #[serde(rename = "stock_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stock_info: Option<StockInfo>,
}
