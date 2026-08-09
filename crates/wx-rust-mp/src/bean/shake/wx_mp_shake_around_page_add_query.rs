//! 对应 Java `bean.shake.WxMpShakeAroundPageAddQuery`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpShakeAroundPageAddQuery {
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "description", default)]
    pub description: String,
    #[serde(rename = "pageUrl", default)]
    pub page_url: String,
    #[serde(rename = "comment", default)]
    pub comment: String,
    #[serde(rename = "iconUrl", default)]
    pub icon_url: String,
}
