//! 对应 Java `me.chanjar.weixin.channel.bean.fund.FlowRelatedInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FlowRelatedInfo {
    #[serde(rename = "related_type", default)]
    pub related_type: i32,
    #[serde(rename = "order_id", default)]
    pub order_id: String,
    #[serde(rename = "aftersale_id", default)]
    pub after_sale_id: String,
    #[serde(rename = "withdraw_id", default)]
    pub withdraw_id: String,
    #[serde(rename = "bookkeeping_time", default)]
    pub bookkeeping_time: String,
    #[serde(rename = "insurance_id", default)]
    pub insurance_id: String,
    #[serde(rename = "transaction_id", default)]
    pub transaction_id: String,
}
