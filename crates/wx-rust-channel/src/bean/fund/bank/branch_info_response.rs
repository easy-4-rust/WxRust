//! 对应 Java `me.chanjar.weixin.channel.bean.fund.bank.BranchInfoResponse.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::fund::*;

#[allow(unused_imports)]
use crate::bean::base::WxChannelBaseResponse;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BranchInfoResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "total_count", default)]
    pub total_count: i32,
    #[serde(rename = "count", default)]
    pub count: i32,
    #[serde(rename = "account_bank", default)]
    pub account_bank: String,
    #[serde(rename = "account_bank_code", default)]
    pub account_bank_code: String,
    #[serde(rename = "bank_alias", default)]
    pub bank_alias: String,
    #[serde(rename = "bank_alias_code", default)]
    pub bank_alias_code: String,
    #[serde(rename = "data", default)]
    pub data: Vec<BranchInfo>,
}
