//! 对应 Java `me.chanjar.weixin.channel.bean.live.dashboard.Ended.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::live::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Ended {
    #[serde(
        rename = "recommend_effective_new_watch_2_uv_over_impression_uv",
        default
    )]
    pub recommend_effective_new_watch2_uv_over_impression_uv: EndedIndexItem,
    #[serde(rename = "average_watch_seconds", default)]
    pub average_watch_seconds: EndedIndexItem,
    #[serde(rename = "comment_uv_over_new_watch_uv", default)]
    pub comment_uv_over_new_watch_uv: EndedIndexItem,
    #[serde(rename = "like_uv_over_new_watch_uv", default)]
    pub like_uv_over_new_watch_uv: EndedIndexItem,
}
