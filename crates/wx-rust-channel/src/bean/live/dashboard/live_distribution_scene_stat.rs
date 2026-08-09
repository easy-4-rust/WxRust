//! 对应 Java `me.chanjar.weixin.channel.bean.live.dashboard.LiveDistributionSceneStat.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::live::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LiveDistributionSceneStat {
    #[serde(rename = "scene_type", default)]
    pub scene_type: i32,
    #[serde(rename = "dist_flow_type_stats", default)]
    pub dist_flow_type_stats: Vec<LiveDistributionByFlowTypeStat>,
    #[serde(rename = "metric_value_total", default)]
    pub metric_value_total: i64,
    #[serde(rename = "gmv", default)]
    pub gmv: i64,
    #[serde(rename = "uv", default)]
    pub uv: i64,
    #[serde(rename = "gmv_per_uv", default)]
    pub gmv_per_uv: i64,
    #[serde(rename = "metric_value", default)]
    pub metric_value: i64,
    #[serde(rename = "metric_value_ratio", default)]
    pub metric_value_ratio: f64,
    #[serde(rename = "pv", default)]
    pub pv: i64,
}
