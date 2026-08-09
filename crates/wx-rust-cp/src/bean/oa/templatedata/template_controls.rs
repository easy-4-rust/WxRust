//! 对应 Java `me.chanjar.weixin.cp.bean.oa.templatedata.TemplateControls.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::oa::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TemplateControls {
    #[serde(rename = "property", default)]
    pub property: crate::bean::oa::wx_cp_oa_approval_template_result::TemplateProperty,
    #[serde(rename = "config", default)]
    pub config: crate::bean::oa::wx_cp_oa_approval_template_result::TemplateConfig,
}
