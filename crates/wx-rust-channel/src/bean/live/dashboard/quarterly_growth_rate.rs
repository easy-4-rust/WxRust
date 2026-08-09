//! 对应 Java `me.chanjar.weixin.channel.bean.live.dashboard.QuarterlyGrowthRate.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::live::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct QuarterlyGrowthRate {
    #[serde(rename = "value", default)]
    pub value: i64,
    #[serde(rename = "is_valid", default)]
    pub is_valid: bool,
}
