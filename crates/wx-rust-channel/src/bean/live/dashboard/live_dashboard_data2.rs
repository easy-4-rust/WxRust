//! 对应 Java `me.chanjar.weixin.channel.bean.live.dashboard.LiveDashboardData2.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::live::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LiveDashboardData2 {
    #[serde(rename = "summary", default)]
    pub summary: LiveDashboardData2Summary,
    #[serde(rename = "source", default)]
    pub source: LiveDashboardData2Source,
    #[serde(rename = "portrait", default)]
    pub portrait: LiveDashboardData2Portrait,
}
