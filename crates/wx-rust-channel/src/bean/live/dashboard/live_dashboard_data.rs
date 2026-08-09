//! 对应 Java `me.chanjar.weixin.channel.bean.live.dashboard.LiveDashboardData.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::live::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LiveDashboardData {
    #[serde(rename = "live_dashboard_data", default)]
    pub live_dashboard_data: LiveDashboardData2,
    #[serde(rename = "live_duration", default)]
    pub live_duration: i64,
    #[serde(rename = "start_time", default)]
    pub start_time: i64,
}
