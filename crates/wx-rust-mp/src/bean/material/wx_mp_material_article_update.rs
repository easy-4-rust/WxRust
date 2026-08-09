//! 对应 Java `bean.material.WxMpMaterialArticleUpdate`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpMaterialArticleUpdate {
    #[serde(rename = "mediaId", default)]
    pub media_id: String,
    #[serde(rename = "index", default)]
    pub index: i32,
    #[serde(rename = "articles", default)]
    pub articles: WxMpNewsArticle,
}
