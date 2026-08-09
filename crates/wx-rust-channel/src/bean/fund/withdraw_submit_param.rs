//! 对应 Java `me.chanjar.weixin.channel.bean.fund.WithdrawSubmitParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WithdrawSubmitParam {
    #[serde(rename = "amount", default)]
    pub amount: i32,
    #[serde(rename = "remark", default)]
    pub remark: String,
    #[serde(rename = "bank_memo", default)]
    pub bank_memo: String,
}
