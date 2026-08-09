//! 对应 Java `com.github.binarywang.wxpay.bean.subscriptionbilling.SubscriptionQueryResult.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SubscriptionQueryResult {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "subscription_id"
    )]
    pub subscription_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "out_trade_no"
    )]
    pub out_trade_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "openid")]
    pub openid: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "description"
    )]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "status")]
    pub status: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "schedule_time"
    )]
    pub schedule_time: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "create_time"
    )]
    pub create_time: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "update_time"
    )]
    pub update_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "amount")]
    pub amount: Option<SubscriptionAmount>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "billing_plan"
    )]
    pub billing_plan: Option<BillingPlan>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "attach")]
    pub attach: Option<String>,
}
