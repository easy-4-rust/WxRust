//! 对应 Java `me.chanjar.weixin.cp.bean.article.NewArticle.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NewArticle {
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "description", default)]
    pub description: String,
    #[serde(rename = "url", default)]
    pub url: String,
    #[serde(rename = "picUrl", default)]
    pub pic_url: String,
    #[serde(rename = "btnText", default)]
    pub btn_text: String,
    #[serde(rename = "appid", default)]
    pub appid: String,
    #[serde(rename = "pagepath", default)]
    pub pagepath: String,
}
