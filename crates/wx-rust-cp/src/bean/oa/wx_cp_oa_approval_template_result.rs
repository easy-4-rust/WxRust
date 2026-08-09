//! 对应 Java `me.chanjar.weixin.cp.bean.oa.WxCpOaApprovalTemplateResult.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpOaApprovalTemplateResult {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "template_names", default)]
    pub template_names: Vec<crate::bean::oa::templatedata::template_title::TemplateTitle>,
    #[serde(rename = "template_content", default)]
    pub template_content: TemplateContent,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TemplateContent {
    #[serde(rename = "controls", default)]
    pub controls: Vec<crate::bean::oa::wx_cp_oa_approval_template_result::TemplateControls>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TemplateControls {
    #[serde(rename = "property", default)]
    pub property: crate::bean::oa::wx_cp_oa_approval_template_result::TemplateProperty,
    #[serde(rename = "config", default)]
    pub config: crate::bean::oa::wx_cp_oa_approval_template_result::TemplateConfig,
}

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
    #[serde(rename = "vacation_list", default)]
    pub vacation_list: crate::bean::oa::templatedata::control::template_vacation::TemplateVacation,
    #[serde(rename = "tips", default)]
    pub tips: crate::bean::oa::templatedata::template_tips::TemplateTips,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TemplateSelector {
    #[serde(rename = "type", default)]
    pub r#type: String,
    #[serde(rename = "options", default)]
    pub options: Vec<crate::bean::oa::wx_cp_oa_approval_template_result::TemplateOption>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TemplateOption {
    #[serde(rename = "key", default)]
    pub key: String,
    #[serde(rename = "value", default)]
    pub value: Vec<crate::bean::oa::templatedata::template_title::TemplateTitle>,
}

impl WxCpOaApprovalTemplateResult {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json)
            .map_err(|e| format!("WxCpOaApprovalTemplateResult 解析失败: {e}"))
    }
}

impl WxCpOaApprovalTemplateResult {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxCpOaApprovalTemplateResult 序列化失败: {e}"))
    }
}
