//! 对应 Java `me.chanjar.weixin.channel.bean.live.dashboard.LiveDistChannelSourceStats.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::live::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LiveDistChannelSourceStats {
    #[serde(rename = "level", default)]
    pub level: i32,
    #[serde(rename = "source_channel_id", default)]
    pub source_channel_id: i64,
    #[serde(rename = "sub_channel_source_stats", default)]
    pub sub_channel_source_stats: Vec<SubLiveDistChannelSourceStats>,
    #[serde(rename = "gmv", default)]
    pub gmv: i64,
    #[serde(rename = "uv", default)]
    pub uv: i64,
    #[serde(rename = "gmv_per_uv", default)]
    pub gmv_per_uv: i64,
    #[serde(rename = "gmv_ratio", default)]
    pub gmv_ratio: f64,
    #[serde(rename = "uv_ratio", default)]
    pub uv_ratio: f64,
    #[serde(rename = "source_channel_name", default)]
    pub source_channel_name: String,
    #[serde(rename = "pv_ratio", default)]
    pub pv_ratio: f64,
}
