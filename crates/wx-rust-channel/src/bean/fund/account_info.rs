//! 对应 Java `me.chanjar.weixin.channel.bean.fund.AccountInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AccountInfo {
    #[serde(rename = "bank_account_type", default)]
    pub bank_account_type: String,
    #[serde(rename = "account_bank", default)]
    pub account_bank: String,
    #[serde(rename = "bank_address_code", default)]
    pub bank_address_code: String,
    #[serde(rename = "bank_branch_id", default)]
    pub bank_branch_id: String,
    #[serde(rename = "bank_name", default)]
    pub bank_name: String,
    #[serde(rename = "account_number", default)]
    pub account_number: String,
    #[serde(rename = "account_bank4show", default)]
    pub account_bank4show: String,
    #[serde(rename = "account_name", default)]
    pub account_name: String,
}
