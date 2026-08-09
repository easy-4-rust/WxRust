//! 对应 Java `me.chanjar.weixin.channel.bean.fund.WithdrawDetailResponse.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[allow(unused_imports)]
use crate::bean::base::WxChannelBaseResponse;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WithdrawDetailResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "amount", default)]
    pub amount: i32,
    #[serde(rename = "create_time", default)]
    pub create_time: i64,
    #[serde(rename = "update_time", default)]
    pub update_time: i64,
    #[serde(rename = "reason", default)]
    pub reason: String,
    #[serde(rename = "remark", default)]
    pub remark: String,
    #[serde(rename = "bank_memo", default)]
    pub bank_memo: String,
    #[serde(rename = "bank_name", default)]
    pub bank_name: String,
    #[serde(rename = "bank_num", default)]
    pub bank_num: String,
    #[serde(rename = "status", default)]
    pub status: String,
}
