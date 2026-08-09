//! 对应 Java `me.chanjar.weixin.cp.bean.article.MpnewsArticle.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MpnewsArticle {
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "thumbMediaId", default)]
    pub thumb_media_id: String,
    #[serde(rename = "author", default)]
    pub author: String,
    #[serde(rename = "contentSourceUrl", default)]
    pub content_source_url: String,
    #[serde(rename = "content", default)]
    pub content: String,
    #[serde(rename = "digest", default)]
    pub digest: String,
    #[serde(rename = "showCoverPic", default)]
    pub show_cover_pic: String,
}
