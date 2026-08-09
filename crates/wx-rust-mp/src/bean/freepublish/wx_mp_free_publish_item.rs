//! 对应 Java `bean.freepublish.WxMpFreePublishItem`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpFreePublishItem {
    #[serde(rename = "article_id", default)]
    pub article_id: String,
    #[serde(rename = "content", default)]
    pub content: WxMpFreePublishInfo,
    #[serde(rename = "update_time", default)]
    pub update_time: String,
}
