//! 对应 Java `com.github.binarywang.wxpay.bean.subscriptionbilling.BillingPlan.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BillingPlan {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "plan_type")]
    pub plan_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "period")]
    pub period: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "total_count"
    )]
    pub total_count: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "executed_count"
    )]
    pub executed_count: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "start_time"
    )]
    pub start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "end_time")]
    pub end_time: Option<String>,
}
