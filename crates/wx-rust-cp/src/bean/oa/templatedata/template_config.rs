//! 对应 Java `me.chanjar.weixin.cp.bean.oa.templatedata.TemplateConfig.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::oa::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TemplateConfig {
    #[serde(rename = "date", default)]
    pub date: crate::bean::oa::templatedata::control::template_date::TemplateDate,
    #[serde(rename = "date_range", default)]
    pub date_range: crate::bean::oa::templatedata::template_date_range::TemplateDateRange,
    #[serde(rename = "selector", default)]
    pub selector: crate::bean::oa::wx_cp_oa_approval_template_result::TemplateSelector,
    #[serde(rename = "contact", default)]
    pub contact: crate::bean::oa::templatedata::control::template_contact::TemplateContact,
    #[serde(rename = "table", default)]
    pub table: crate::bean::oa::templatedata::control::template_table::TemplateTable,
    #[serde(rename = "attendance", default)]
    pub attendance: crate::bean::oa::templatedata::control::template_attendance::TemplateAttendance,
    #[serde(rename = "location", default)]
    pub location: crate::bean::oa::templatedata::template_location::TemplateLocation,
    #[serde(rename = "vacation_list", default)]
    pub vacation_list: crate::bean::oa::templatedata::control::template_vacation::TemplateVacation,
    #[serde(rename = "tips", default)]
    pub tips: crate::bean::oa::templatedata::template_tips::TemplateTips,
}
