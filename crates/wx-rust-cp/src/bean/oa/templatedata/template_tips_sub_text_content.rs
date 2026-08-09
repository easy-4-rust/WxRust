//! 对应 Java `me.chanjar.weixin.cp.bean.oa.templatedata.TemplateTipsSubTextContent.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::oa::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TemplateTipsSubTextContent {
    #[serde(rename = "plain_text", default)]
    pub plain_text: crate::bean::oa::templatedata::template_tips_sub_text_content_plain_text::TemplateTipsSubTextContentPlainText,
    #[serde(rename = "link", default)]
    pub link: crate::bean::oa::templatedata::template_tips_sub_text_content_link::TemplateTipsSubTextContentLink,
}
