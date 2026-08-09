//! 对应 Java `com.github.binarywang.wxpay.bean.subscriptionbilling.SubscriptionCancelResult.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SubscriptionCancelResult {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "subscription_id"
    )]
    pub subscription_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "status")]
    pub status: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "cancel_time"
    )]
    pub cancel_time: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "cancel_reason"
    )]
    pub cancel_reason: Option<String>,
}
