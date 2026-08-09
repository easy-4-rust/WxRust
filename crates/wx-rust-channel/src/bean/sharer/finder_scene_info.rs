//! 对应 Java `me.chanjar.weixin.channel.bean.sharer.FinderSceneInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FinderSceneInfo {
    #[serde(rename = "promoter_id", default)]
    pub promoter_id: String,
    #[serde(rename = "finder_nickname", default)]
    pub finder_nickname: String,
    #[serde(rename = "live_export_id", default)]
    pub live_export_id: String,
    #[serde(rename = "video_export_id", default)]
    pub video_export_id: String,
    #[serde(rename = "video_title", default)]
    pub video_title: String,
}
