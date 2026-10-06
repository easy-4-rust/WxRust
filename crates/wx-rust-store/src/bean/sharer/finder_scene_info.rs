//! 对应 Java `com.binarywang.wxjava.store.bean.sharer.FinderSceneInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 FinderSceneInfo；对应 Java com.binarywang.wxjava.store.bean.sharer.FinderSceneInfo.java。
pub struct FinderSceneInfo {
    #[serde(rename = "promoter_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promoter_id: Option<String>,
    #[serde(rename = "finder_nickname", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finder_nickname: Option<String>,
    #[serde(rename = "live_export_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_export_id: Option<String>,
    #[serde(rename = "video_export_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_export_id: Option<String>,
    #[serde(rename = "video_title", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_title: Option<String>,
}
