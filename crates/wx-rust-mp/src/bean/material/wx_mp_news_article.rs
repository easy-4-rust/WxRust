//! 对应 Java `bean.material.WxMpNewsArticle`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpNewsArticle {
    #[serde(rename = "thumbMediaId", default)]
    pub thumb_media_id: String,
    #[serde(rename = "thumbUrl", default)]
    pub thumb_url: String,
    #[serde(rename = "author", default)]
    pub author: String,
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "contentSourceUrl", default)]
    pub content_source_url: String,
    #[serde(rename = "content", default)]
    pub content: String,
    #[serde(rename = "digest", default)]
    pub digest: String,
    #[serde(rename = "showCoverPic", default)]
    pub show_cover_pic: bool,
    #[serde(rename = "url", default)]
    pub url: String,
    #[serde(rename = "needOpenComment", default)]
    pub need_open_comment: bool,
    #[serde(rename = "onlyFansCanComment", default)]
    pub only_fans_can_comment: bool,
}
