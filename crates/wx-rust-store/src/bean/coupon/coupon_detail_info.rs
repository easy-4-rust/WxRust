//! 对应 Java `com.binarywang.wxjava.store.bean.coupon.CouponDetailInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::{DiscountInfo, ExtInfo, PromoteInfo, ReceiveInfo, ValidInfo};

/// 微信小店 CouponDetailInfo 数据类型；对应 Java com.binarywang.wxjava.store.bean.coupon.CouponDetailInfo.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CouponDetailInfo {
    #[serde(rename = "name", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "valid_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_info: Option<ValidInfo>,
    #[serde(rename = "promote_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promote_info: Option<PromoteInfo>,
    #[serde(rename = "discount_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_info: Option<DiscountInfo>,
    #[serde(rename = "ext_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_info: Option<ExtInfo>,
    #[serde(rename = "receive_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receive_info: Option<ReceiveInfo>,
}
