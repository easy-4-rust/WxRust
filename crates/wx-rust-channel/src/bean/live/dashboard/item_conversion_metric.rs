//! 对应 Java `me.chanjar.weixin.channel.bean.live.dashboard.ItemConversionMetric.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::live::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ItemConversionMetric {
    #[serde(rename = "metric_value", default)]
    pub metric_value: f64,
    #[serde(rename = "median_to_recent_7_days", default)]
    pub median_to_recent7_days: f64,
    #[serde(rename = "within_industry_percentage", default)]
    pub within_industry_percentage: f64,
    #[serde(rename = "quarterly_growth_rate", default)]
    pub quarterly_growth_rate: QuarterlyGrowthRate,
}
