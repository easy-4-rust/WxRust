//! 对应 Java `me.chanjar.weixin.cp.bean.oa.templatedata.TemplateProperty.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::oa::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TemplateProperty {
    #[serde(rename = "control", default)]
    pub control: String,
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "title", default)]
    pub title: Vec<crate::bean::oa::templatedata::template_title::TemplateTitle>,
    #[serde(rename = "placeholder", default)]
    pub placeholder: Vec<crate::bean::oa::templatedata::template_title::TemplateTitle>,
    #[serde(rename = "require", default)]
    pub require: i32,
    #[serde(rename = "un_print", default)]
    pub un_print: i32,
    #[serde(rename = "config", default)]
    pub config: crate::bean::oa::wx_cp_oa_approval_template_result::TemplateConfig,
}
