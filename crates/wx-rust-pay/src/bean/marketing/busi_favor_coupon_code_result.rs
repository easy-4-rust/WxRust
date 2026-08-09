//! 对应 Java `com.github.binarywang.wxpay.bean.marketing.BusiFavorCouponCodeResult.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BusiFavorCouponCodeResult {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "stock_id")]
    pub stock_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "total_count"
    )]
    pub total_count: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "success_count"
    )]
    pub success_count: Option<i32>,
    #[serde(default, rename = "success_codes")]
    pub success_codes: Vec<Option<String>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "success_time"
    )]
    pub success_time: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "fail_count"
    )]
    pub fail_count: Option<i32>,
    #[serde(default, rename = "fail_codes")]
    pub fail_codes: Vec<FailCode>,
    #[serde(default, rename = "exist_codes")]
    pub exist_codes: Vec<Option<String>>,
    #[serde(default, rename = "duplicate_codes")]
    pub duplicate_codes: Vec<Option<String>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FailCode {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "coupon_code"
    )]
    pub coupon_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "code")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "message")]
    pub message: Option<String>,
}
