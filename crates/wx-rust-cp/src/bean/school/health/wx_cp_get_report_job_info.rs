//! 对应 Java `me.chanjar.weixin.cp.bean.school.health.WxCpGetReportJobInfo.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::school::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpGetReportJobInfo {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "job_info", default)]
    pub job_info: JobInfo,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct JobInfo {
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "creator", default)]
    pub creator: String,
    #[serde(rename = "type", default)]
    pub r#type: i32,
    #[serde(rename = "report_type", default)]
    pub report_type: i32,
    #[serde(rename = "skip_weekend", default)]
    pub skip_weekend: i32,
    #[serde(rename = "finish_cnt", default)]
    pub finish_cnt: i32,
    #[serde(rename = "apply_range", default)]
    pub apply_range: crate::bean::school::health::wx_cp_get_report_job_info::ApplyRange,
    #[serde(rename = "report_to", default)]
    pub report_to: crate::bean::school::health::wx_cp_get_report_job_info::ReportTo,
    #[serde(rename = "question_templates", default)]
    pub question_templates:
        Vec<crate::bean::school::health::wx_cp_get_report_job_info::QuestionTemplate>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApplyRange {
    #[serde(rename = "userids", default)]
    pub user_ids: Vec<String>,
    #[serde(rename = "partyids", default)]
    pub party_ids: Vec<i32>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ReportTo {
    #[serde(rename = "userids", default)]
    pub user_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct QuestionTemplate {
    #[serde(rename = "question_id", default)]
    pub question_id: i32,
    #[serde(rename = "question_type", default)]
    pub question_type: i32,
    #[serde(rename = "is_required", default)]
    pub is_required: i32,
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "option_list", default)]
    pub option_list: Vec<crate::bean::school::health::wx_cp_get_report_job_info::OptionList>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OptionList {
    #[serde(rename = "option_id", default)]
    pub option_id: i32,
    #[serde(rename = "option_text", default)]
    pub option_text: String,
}

impl WxCpGetReportJobInfo {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpGetReportJobInfo 解析失败: {e}"))
    }
}

impl WxCpGetReportJobInfo {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpGetReportJobInfo 序列化失败: {e}"))
    }
}
