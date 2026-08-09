//! 对应 Java `me.chanjar.weixin.channel.bean.live.dashboard.LiveDistributionChannel.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::live::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LiveDistributionChannel {
    #[serde(rename = "audience_count", default)]
    pub audience_count: i64,
    #[serde(rename = "total_joinlive_count", default)]
    pub total_joinlive_count: i64,
    #[serde(rename = "live_dist_channel_source_by_scene_stats", default)]
    pub live_dist_channel_source_by_scene_stats: Vec<LiveDistributionSceneStat>,
    #[serde(rename = "live_dist_channel_source_stats", default)]
    pub live_dist_channel_source_stats: Vec<LiveDistributionByFlowTypeStat>,
    #[serde(rename = "data_key", default)]
    pub data_key: Vec<String>,
}
