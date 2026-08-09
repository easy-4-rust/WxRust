//! 对应 Java `com.github.binarywang.wxpay.bean.ecommerce.ReturnAdvanceResult.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ReturnAdvanceResult {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "refund_id")]
    pub refund_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "advance_return_id"
    )]
    pub advance_return_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "return_amount"
    )]
    pub return_amount: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "payer_mchid"
    )]
    pub payer_mchid: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "payer_account"
    )]
    pub payer_account: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "payee_mchid"
    )]
    pub payee_mchid: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "payee_account"
    )]
    pub payee_account: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "result")]
    pub result: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "success_time"
    )]
    pub success_time: Option<String>,
}
