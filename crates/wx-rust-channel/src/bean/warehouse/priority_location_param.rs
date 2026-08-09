//! 对应 Java `me.chanjar.weixin.channel.bean.warehouse.PriorityLocationParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PriorityLocationParam {
    #[serde(rename = "address_id1", default)]
    pub address_id1: i32,
    #[serde(rename = "address_id2", default)]
    pub address_id2: i32,
    #[serde(rename = "address_id3", default)]
    pub address_id3: i32,
    #[serde(rename = "address_id4", default)]
    pub address_id4: i32,
    #[serde(rename = "priority_sort", default)]
    pub priority_sort: Vec<String>,
}
