//! 对应 Java `com.github.binarywang.wxpay.bean.bank.BankBranchesResult.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BankBranchesResult {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "total_count"
    )]
    pub total_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "count")]
    pub count: Option<i32>,
    #[serde(default, rename = "data")]
    pub data: Vec<BankBranch>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "offset")]
    pub offset: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "links")]
    pub links: Option<PageLink>,
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
        rename = "bank_alias"
    )]
    pub bank_alias: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "bank_alias_code"
    )]
    pub bank_alias_code: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BankBranch {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "bank_branch_name"
    )]
    pub bank_branch_name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "bank_branch_id"
    )]
    pub bank_branch_id: Option<String>,
}
