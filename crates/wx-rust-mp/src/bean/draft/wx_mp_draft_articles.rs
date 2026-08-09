//! 对应 Java `bean.draft.WxMpDraftArticles`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpDraftArticles {
    #[serde(rename = "article_type", default)]
    pub article_type: String,
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "author", default)]
    pub author: String,
    #[serde(rename = "digest", default)]
    pub digest: String,
    #[serde(rename = "content", default)]
    pub content: String,
    #[serde(rename = "content_source_url", default)]
    pub content_source_url: String,
    #[serde(rename = "thumb_media_id", default)]
    pub thumb_media_id: String,
    #[serde(rename = "show_cover_pic", default)]
    pub show_cover_pic: i32,
    #[serde(rename = "need_open_comment", default)]
    pub need_open_comment: i32,
    #[serde(rename = "only_fans_can_comment", default)]
    pub only_fans_can_comment: i32,
    #[serde(rename = "url", default)]
    pub url: String,
    #[serde(rename = "thumb_url", default)]
    pub thumb_url: String,
    #[serde(rename = "pic_crop_235_1", default)]
    pub pic_crop2351: String,
    #[serde(rename = "pic_crop_1_1", default)]
    pub pic_crop11: String,
    #[serde(rename = "image_info", default)]
    pub image_info: WxMpDraftImageInfo,
    #[serde(rename = "cover_info", default)]
    pub cover_info: WxMpDraftCoverInfo,
    #[serde(rename = "product_info", default)]
    pub product_info: WxMpDraftProductInfo,
}
