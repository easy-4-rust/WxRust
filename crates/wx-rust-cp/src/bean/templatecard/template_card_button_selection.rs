//! 对应 Java `me.chanjar.weixin.cp.bean.templatecard.TemplateCardButtonSelection.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TemplateCardButtonSelection {
    #[serde(rename = "questionKey", default)]
    pub question_key: String,
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "selectedId", default)]
    pub selected_id: String,
    #[serde(rename = "optionList", default)]
    pub option_list: Vec<crate::bean::templatecard::template_card_button_selection_option::TemplateCardButtonSelectionOption>,
}
