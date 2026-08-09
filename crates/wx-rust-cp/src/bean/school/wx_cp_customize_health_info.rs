//! 对应 Java `me.chanjar.weixin.cp.bean.school.WxCpCustomizeHealthInfo.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpCustomizeHealthInfo {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "health_infos", default)]
    pub health_infos: Vec<HealthInfo>,
    #[serde(rename = "template_id", default)]
    pub template_id: String,
    #[serde(rename = "next_key", default)]
    pub next_key: String,
    #[serde(rename = "ending", default)]
    pub ending: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct HealthInfo {
    #[serde(rename = "userid", default)]
    pub user_id: String,
    #[serde(rename = "health_qrcode_status", default)]
    pub health_qr_code_status: i32,
    #[serde(rename = "self_submit", default)]
    pub self_submit: i32,
    #[serde(rename = "report_values", default)]
    pub report_values: Vec<crate::bean::school::wx_cp_customize_health_info::ReportValue>,
    #[serde(rename = "question_templates", default)]
    pub question_templates: Vec<crate::bean::school::wx_cp_customize_health_info::QuestionTemplate>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ReportValue {
    #[serde(rename = "question_id", default)]
    pub question_id: i32,
    #[serde(rename = "single_chose", default)]
    pub single_chose: i32,
    #[serde(rename = "text", default)]
    pub text: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct QuestionTemplate {
    #[serde(rename = "question_id", default)]
    pub question_id: i32,
    #[serde(rename = "question_type", default)]
    pub question_type: i32,
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "is_must_fill", default)]
    pub is_must_fill: i32,
    #[serde(rename = "is_not_display", default)]
    pub is_not_display: i32,
    #[serde(rename = "option_list", default)]
    pub option_list: Vec<crate::bean::school::wx_cp_customize_health_info::OptionList>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OptionList {
    #[serde(rename = "option_id", default)]
    pub option_id: i32,
    #[serde(rename = "option_text", default)]
    pub option_text: String,
}

impl WxCpCustomizeHealthInfo {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpCustomizeHealthInfo 解析失败: {e}"))
    }
}

impl WxCpCustomizeHealthInfo {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpCustomizeHealthInfo 序列化失败: {e}"))
    }
}
