//! 对应 Java `com.binarywang.wxjava.store.bean.fund.bank.BranchInfoResponse.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::BranchInfo;

/// 微信小店 BranchInfoResponse 数据类型；对应 Java com.binarywang.wxjava.store.bean.fund.bank.BranchInfoResponse.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BranchInfoResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "total_count", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    #[serde(rename = "count", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename = "account_bank", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_bank: Option<String>,
    #[serde(rename = "account_bank_code", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_bank_code: Option<String>,
    #[serde(rename = "bank_alias", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_alias: Option<String>,
    #[serde(rename = "bank_alias_code", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_alias_code: Option<String>,
    #[serde(rename = "data", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<BranchInfo>>,
}
