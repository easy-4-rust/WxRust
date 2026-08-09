//! 对应 Java `com.github.binarywang.wxpay.bean.bank.BankInfo.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BankInfo {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "bank_alias"
    )]
    pub bank_alias: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "bank_alias_code"
    )]
    pub bank_alias_code: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "account_bank"
    )]
    pub account_bank: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "account_bank_code"
    )]
    pub account_bank_code: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "need_bank_branch"
    )]
    pub need_bank_branch: Option<bool>,
}
