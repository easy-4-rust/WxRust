//! 对应 Java `com.github.binarywang.wxpay.bean.applyment.SettlementApplicationResult.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SettlementApplicationResult {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "account_name"
    )]
    pub account_name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "account_type"
    )]
    pub account_type: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "account_bank"
    )]
    pub account_bank: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "bank_name")]
    pub bank_name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "bank_branch_id"
    )]
    pub bank_branch_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "account_number"
    )]
    pub account_number: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "verify_result"
    )]
    pub verify_result: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "verify_fail_reason"
    )]
    pub verify_fail_reason: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "verify_finish_time"
    )]
    pub verify_finish_time: Option<String>,
}
