//! 对应 Java `com.binarywang.wxjava.store.bean.fund.AccountInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 AccountInfo；对应 Java com.binarywang.wxjava.store.bean.fund.AccountInfo.java。
pub struct AccountInfo {
    #[serde(rename = "bank_account_type", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account_type: Option<String>,
    #[serde(rename = "account_bank", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_bank: Option<String>,
    #[serde(rename = "bank_address_code", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_address_code: Option<String>,
    #[serde(rename = "bank_branch_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_branch_id: Option<String>,
    #[serde(rename = "bank_name", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    #[serde(rename = "account_number", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    #[serde(rename = "account_bank4show", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_bank4show: Option<String>,
    #[serde(rename = "account_name", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
}
