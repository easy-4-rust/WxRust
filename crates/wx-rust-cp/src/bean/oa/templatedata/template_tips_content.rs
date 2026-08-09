//! 对应 Java `me.chanjar.weixin.cp.bean.oa.templatedata.TemplateTipsContent.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::oa::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TemplateTipsContent {
    #[serde(rename = "text", default)]
    pub text: crate::bean::oa::templatedata::template_tips_text::TemplateTipsText,
    #[serde(rename = "lang", default)]
    pub lang: String,
}
