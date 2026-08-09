//! 对应 Java `me.chanjar.weixin.channel.bean.live.dashboard.OnAirIndexItem.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::live::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OnAirIndexItem {
    #[serde(rename = "n", default)]
    pub n: i32,
    #[serde(rename = "last_n_mins_value", default)]
    pub last_n_mins_value: i32,
    #[serde(rename = "last_2n_to_n_mins_value", default)]
    pub last2n_to_n_mins_value: i32,
    #[serde(rename = "last_n_mins_percentile", default)]
    pub last_n_mins_percentile: i32,
    #[serde(rename = "value", default)]
    pub value: i64,
    #[serde(rename = "percentile", default)]
    pub percentile: i32,
}
