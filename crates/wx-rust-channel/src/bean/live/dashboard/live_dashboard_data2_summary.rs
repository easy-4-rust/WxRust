//! 对应 Java `me.chanjar.weixin.channel.bean.live.dashboard.LiveDashboardData2Summary.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::live::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LiveDashboardData2Summary {
    #[serde(rename = "new_watch_uv", default)]
    pub new_watch_uv: i64,
    #[serde(rename = "max_online_watch_uv", default)]
    pub max_online_watch_uv: i64,
    #[serde(rename = "impression_uv", default)]
    pub impression_uv: i64,
    #[serde(rename = "average_watch_seconds_per_audience", default)]
    pub average_watch_seconds_per_audience: i64,
    #[serde(rename = "new_follow_uv", default)]
    pub new_follow_uv: i64,
    #[serde(rename = "new_fans_club_uv", default)]
    pub new_fans_club_uv: i64,
    #[serde(rename = "comment_uv", default)]
    pub comment_uv: i64,
    #[serde(rename = "reward_uv", default)]
    pub reward_uv: i64,
    #[serde(rename = "sharing_uv", default)]
    pub sharing_uv: i64,
    #[serde(rename = "hot_quota", default)]
    pub hot_quota: i64,
}
