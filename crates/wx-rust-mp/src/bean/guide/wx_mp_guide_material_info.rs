//! 对应 Java `bean.guide.WxMpGuideMaterialInfo`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpGuideMaterialInfo {
    #[serde(rename = "type", default)]
    pub r#type: i32,
    #[serde(rename = "media_id", default)]
    pub media_id: String,
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "path", default)]
    pub path: String,
    #[serde(rename = "appid", default)]
    pub app_id: String,
    #[serde(rename = "word", default)]
    pub word: String,
}
