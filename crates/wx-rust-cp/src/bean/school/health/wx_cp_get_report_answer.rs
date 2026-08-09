//! 对应 Java `me.chanjar.weixin.cp.bean.school.health.WxCpGetReportAnswer.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::school::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpGetReportAnswer {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "answers", default)]
    pub answers: Vec<Answer>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Answer {
    #[serde(rename = "userid", default)]
    pub user_id: String,
    #[serde(rename = "id_type", default)]
    pub id_type: i32,
    #[serde(rename = "report_time", default)]
    pub report_time: i64,
    #[serde(rename = "student_userid", default)]
    pub student_user_id: String,
    #[serde(rename = "parent_userid", default)]
    pub parent_user_id: String,
    #[serde(rename = "report_values", default)]
    pub report_values: Vec<crate::bean::school::health::wx_cp_get_report_answer::ReportValue>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ReportValue {
    #[serde(rename = "question_id", default)]
    pub question_id: i32,
    #[serde(rename = "single_choice", default)]
    pub single_choice: i32,
    #[serde(rename = "multi_choice", default)]
    pub multi_choice: Vec<i32>,
    #[serde(rename = "text", default)]
    pub text: String,
    #[serde(rename = "fileid", default)]
    pub file_id: Vec<String>,
}

impl WxCpGetReportAnswer {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpGetReportAnswer 解析失败: {e}"))
    }
}

impl WxCpGetReportAnswer {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpGetReportAnswer 序列化失败: {e}"))
    }
}
