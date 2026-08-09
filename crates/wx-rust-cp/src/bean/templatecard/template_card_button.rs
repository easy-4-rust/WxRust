//! 对应 Java `me.chanjar.weixin.cp.bean.templatecard.TemplateCardButton.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TemplateCardButton {
    #[serde(rename = "text", default)]
    pub text: String,
    #[serde(rename = "style", default)]
    pub style: i32,
    #[serde(rename = "key", default)]
    pub key: String,
    #[serde(rename = "type", default)]
    pub r#type: i32,
    #[serde(rename = "url", default)]
    pub url: String,
}
