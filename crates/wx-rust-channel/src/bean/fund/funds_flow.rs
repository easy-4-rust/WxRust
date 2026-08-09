//! 对应 Java `me.chanjar.weixin.channel.bean.fund.FundsFlow.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FundsFlow {
    #[serde(rename = "flow_id", default)]
    pub flow_id: String,
    #[serde(rename = "funds_type", default)]
    pub funds_type: i32,
    #[serde(rename = "flow_type", default)]
    pub flow_type: i32,
    #[serde(rename = "amount", default)]
    pub amount: i32,
    #[serde(rename = "balance", default)]
    pub balance: i32,
    #[serde(rename = "related_info_list", default)]
    pub related_infos: Vec<FlowRelatedInfo>,
    #[serde(rename = "bookkeeping_time", default)]
    pub bookkeeping_time: String,
    #[serde(rename = "remark", default)]
    pub remark: String,
}
