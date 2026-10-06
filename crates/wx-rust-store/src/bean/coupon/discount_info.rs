//! 对应 Java `com.binarywang.wxjava.store.bean.coupon.DiscountInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::DiscountCondition;

/// 微信小店 DiscountInfo 数据类型；对应 Java com.binarywang.wxjava.store.bean.coupon.DiscountInfo.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DiscountInfo {
    #[serde(rename = "discount_num", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_num: Option<i32>,
    #[serde(rename = "discount_fee", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_fee: Option<i32>,
    #[serde(rename = "discount_condition", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_condition: Option<DiscountCondition>,
}
