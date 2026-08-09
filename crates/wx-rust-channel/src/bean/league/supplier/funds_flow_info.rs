//! 对应 Java `me.chanjar.weixin.channel.bean.league.supplier.FundsFlowInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::league::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FundsFlowInfo {
    #[serde(rename = "flow_id", default)]
    pub flow_id: String,
    #[serde(rename = "funds_type", default)]
    pub funds_type: i32,
    #[serde(rename = "amount", default)]
    pub amount: i32,
    #[serde(rename = "balance", default)]
    pub balance: i32,
    #[serde(rename = "bookkeeping_time", default)]
    pub bookkeeping_time: String,
    #[serde(rename = "remark", default)]
    pub remark: String,
    #[serde(rename = "order_id", default)]
    pub order_id: String,
    #[serde(rename = "withdraw_id", default)]
    pub withdraw_id: String,
}
