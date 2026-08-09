//! 对应 Java `com.github.binarywang.wxpay.bean.marketing.BusiFavorStocksBudgetRequest.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BusiFavorStocksBudgetRequest {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "stock_id")]
    pub stock_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "target_max_coupons"
    )]
    pub target_max_coupons: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "target_max_coupons_by_day"
    )]
    pub target_max_coupons_by_day: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "current_max_coupons"
    )]
    pub current_max_coupons: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "current_max_coupons_by_day"
    )]
    pub current_max_coupons_by_day: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "modify_budget_request_no"
    )]
    pub modify_budget_request_no: Option<String>,
}
