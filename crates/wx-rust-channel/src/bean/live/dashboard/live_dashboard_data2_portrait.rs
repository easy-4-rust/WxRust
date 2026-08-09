//! 对应 Java `me.chanjar.weixin.channel.bean.live.dashboard.LiveDashboardData2Portrait.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::live::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LiveDashboardData2Portrait {
    #[serde(rename = "online_watch_uv", default)]
    pub online_watch_uv: Vec<Series>,
    #[serde(rename = "new_watch_uv", default)]
    pub new_watch_uv: Vec<Series>,
}
