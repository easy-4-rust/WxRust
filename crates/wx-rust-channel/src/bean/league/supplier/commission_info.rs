//! 对应 Java `me.chanjar.weixin.channel.bean.league.supplier.CommissionInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::league::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CommissionInfo {
    #[serde(rename = "status", default)]
    pub status: i32,
    #[serde(rename = "service_ratio", default)]
    pub service_ratio: i32,
    #[serde(rename = "ratio", default)]
    pub ratio: i32,
    #[serde(rename = "start_time", default)]
    pub start_time: i64,
    #[serde(rename = "end_time", default)]
    pub end_time: i64,
    #[serde(rename = "link", default)]
    pub link: String,
}
