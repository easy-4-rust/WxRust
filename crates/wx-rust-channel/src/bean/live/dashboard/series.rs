//! 对应 Java `me.chanjar.weixin.channel.bean.live.dashboard.Series.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::live::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Series {
    #[serde(rename = "points", default)]
    pub points: Vec<Point>,
    #[serde(rename = "dimensions", default)]
    pub dimensions: Vec<Dimension>,
    #[serde(rename = "step", default)]
    pub step: i64,
    #[serde(rename = "begin_ts", default)]
    pub begin_ts: i64,
    #[serde(rename = "end_ts", default)]
    pub end_ts: i64,
}
