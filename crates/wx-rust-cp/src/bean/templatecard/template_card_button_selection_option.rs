//! 对应 Java `me.chanjar.weixin.cp.bean.templatecard.TemplateCardButtonSelectionOption.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TemplateCardButtonSelectionOption {
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "text", default)]
    pub text: String,
}
