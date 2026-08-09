//! 对应 Java `com.github.binarywang.wxpay.bean.marketing.payroll.PartnerIncomeRecordsResult.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PartnerIncomeRecordsResult {
    #[serde(default, rename = "total_count")]
    pub total_count: i32,
    #[serde(default, rename = "offset")]
    pub offset: i32,
    #[serde(default, rename = "limit")]
    pub limit: i32,
    #[serde(default, rename = "data")]
    pub income_record_data_list: Vec<IncomeRecordData>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct IncomeRecordData {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sub_mchid")]
    pub sub_mchid: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "account_type"
    )]
    pub account_type: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "income_record_type"
    )]
    pub income_record_type: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "income_record_id"
    )]
    pub income_record_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "amount")]
    pub amount: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "success_time"
    )]
    pub success_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "bank_name")]
    pub bank_name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "bank_account_name"
    )]
    pub bank_account_name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "bank_account_number"
    )]
    pub bank_account_number: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "recharge_remark"
    )]
    pub recharge_remark: Option<String>,
    #[serde(default, rename = "links")]
    pub links_data_list: Vec<LinksData>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LinksData {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "next")]
    pub next: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "prev")]
    pub prev: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "self")]
    pub self_: Option<String>,
}
