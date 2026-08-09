//! 对应 Java `bean.template.WxMpTemplate`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpTemplate {
    #[serde(rename = "template_id", default)]
    pub template_id: String,
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "primary_industry", default)]
    pub primary_industry: String,
    #[serde(rename = "deputy_industry", default)]
    pub deputy_industry: String,
    #[serde(rename = "content", default)]
    pub content: String,
    #[serde(rename = "example", default)]
    pub example: String,
}
