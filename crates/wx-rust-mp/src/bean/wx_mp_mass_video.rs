//! 对应 Java `bean.WxMpMassVideo`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpMassVideo {
    #[serde(rename = "mediaId", default)]
    pub media_id: String,
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "description", default)]
    pub description: String,
}
