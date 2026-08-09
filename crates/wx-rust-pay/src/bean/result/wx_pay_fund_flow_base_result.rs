//! 对应 Java `com.github.binarywang.wxpay.bean.result.WxPayFundFlowBaseResult.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxPayFundFlowBaseResult {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "BillingTime"
    )]
    pub billing_time: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "bizTransactionId"
    )]
    pub biz_transaction_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "fundFlowId"
    )]
    pub fund_flow_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "bizName")]
    pub biz_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "bizType")]
    pub biz_type: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "financialType"
    )]
    pub financial_type: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "financialFee"
    )]
    pub financial_fee: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "AccountBalance"
    )]
    pub account_balance: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "fundApplicant"
    )]
    pub fund_applicant: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "memo")]
    pub memo: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "bizVoucherId"
    )]
    pub biz_voucher_id: Option<String>,
}
