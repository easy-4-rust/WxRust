//! 对应 Java `me.chanjar.weixin.channel.bean.fund.bank.BankInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::fund::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BankInfo {
    #[serde(rename = "account_bank", default)]
    pub account_bank: String,
    #[serde(rename = "bank_code", default)]
    pub bank_code: String,
    #[serde(rename = "bank_id", default)]
    pub bank_id: String,
    #[serde(rename = "bank_name", default)]
    pub bank_name: String,
    #[serde(rename = "bank_type", default)]
    pub bank_type: i32,
    #[serde(rename = "need_branch", default)]
    pub need_branch: bool,
    #[serde(rename = "branch_id", default)]
    pub branch_id: String,
}
