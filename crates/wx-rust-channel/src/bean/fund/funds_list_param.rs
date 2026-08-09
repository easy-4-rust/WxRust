//! 对应 Java `me.chanjar.weixin.channel.bean.fund.FundsListParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FundsListParam {
    #[serde(rename = "page", default)]
    pub page: i32,
    #[serde(rename = "page_size", default)]
    pub page_size: i32,
    #[serde(rename = "start_time", default)]
    pub start_time: i64,
    #[serde(rename = "end_time", default)]
    pub end_time: i64,
    #[serde(rename = "flow_type", default)]
    pub flow_type: i32,
    #[serde(rename = "transaction_id", default)]
    pub transaction_id: String,
    #[serde(rename = "next_key", default)]
    pub next_key: String,
}
