//! 对应 Java `me.chanjar.weixin.channel.bean.live.dashboard.LiveComparisonIndex.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::live::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LiveComparisonIndex {
    #[serde(rename = "is_living", default)]
    pub is_living: bool,
    #[serde(rename = "on_air", default)]
    pub on_air: OnAir,
    #[serde(rename = "ended", default)]
    pub ended: Ended,
}
