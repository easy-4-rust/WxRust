//! 对应 Java `com.binarywang.wxjava.store.bean.fund.bank.BankInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 BankInfo；对应 Java com.binarywang.wxjava.store.bean.fund.bank.BankInfo.java。
pub struct BankInfo {
    #[serde(rename = "account_bank", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_bank: Option<String>,
    #[serde(rename = "bank_code", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_code: Option<String>,
    #[serde(rename = "bank_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_id: Option<String>,
    #[serde(rename = "bank_name", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    #[serde(rename = "bank_type", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_type: Option<i32>,
    #[serde(rename = "need_branch", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_branch: Option<bool>,
    #[serde(rename = "branch_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_id: Option<String>,
}
