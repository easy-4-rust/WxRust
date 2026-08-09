//! 对应 Java `me.chanjar.weixin.channel.bean.live.dashboard.LiveEcConversionMetric.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::live::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LiveEcConversionMetric {
    #[serde(rename = "recent_10_min_conversion", default)]
    pub recent10_min_conversion: ConversionMetric,
    #[serde(rename = "whole_live_conversion", default)]
    pub whole_live_conversion: ConversionMetric,
}
