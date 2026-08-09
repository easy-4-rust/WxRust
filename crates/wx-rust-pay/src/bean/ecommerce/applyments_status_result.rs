//! 对应 Java `com.github.binarywang.wxpay.bean.ecommerce.ApplymentsStatusResult.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApplymentsStatusResult {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "applyment_state"
    )]
    pub applyment_state: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "applyment_state_desc"
    )]
    pub applyment_state_desc: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sign_url")]
    pub sign_url: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "sign_state"
    )]
    pub sign_state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sub_mchid")]
    pub sub_mchid: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "account_validation"
    )]
    pub account_validation: Option<AccountValidation>,
    #[serde(default, rename = "audit_detail")]
    pub audit_detail: Vec<AuditDetail>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "legal_validation_url"
    )]
    pub legal_validation_url: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "out_request_no"
    )]
    pub out_request_no: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "applyment_id"
    )]
    pub applyment_id: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AccountValidation {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "account_name"
    )]
    pub account_name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "account_no"
    )]
    pub account_no: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "pay_amount"
    )]
    pub pay_amount: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "destination_account_number"
    )]
    pub destination_account_number: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "destination_account_name"
    )]
    pub destination_account_name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "destination_account_bank"
    )]
    pub destination_account_bank: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "city")]
    pub city: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "remark")]
    pub remark: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deadline")]
    pub deadline: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AuditDetail {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "param_name"
    )]
    pub param_name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "reject_reason"
    )]
    pub reject_reason: Option<String>,
}
