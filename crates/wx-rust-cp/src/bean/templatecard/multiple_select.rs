//! 对应 Java `me.chanjar.weixin.cp.bean.templatecard.MultipleSelect.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MultipleSelect {
    #[serde(rename = "question_key", default)]
    pub question_key: String,
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "selected_id", default)]
    pub selected_id: String,
    #[serde(rename = "options", default)]
    pub options: Vec<crate::bean::templatecard::checkbox_option::CheckboxOption>,
}
