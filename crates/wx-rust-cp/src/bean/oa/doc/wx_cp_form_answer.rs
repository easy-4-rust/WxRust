//! 对应 Java `me.chanjar.weixin.cp.bean.oa.doc.WxCpFormAnswer.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::oa::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpFormAnswer {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "answer", default)]
    pub answer: Answer,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Answer {
    #[serde(rename = "answer_list", default)]
    pub answer_list: Vec<crate::bean::oa::doc::wx_cp_form_answer::AnswerItem>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AnswerItem {
    #[serde(rename = "answer_id", default)]
    pub answer_id: i64,
    #[serde(rename = "user_name", default)]
    pub user_name: String,
    #[serde(rename = "ctime", default)]
    pub ctime: i64,
    #[serde(rename = "mtime", default)]
    pub mtime: i64,
    #[serde(rename = "reply", default)]
    pub reply: crate::bean::oa::doc::wx_cp_form_answer::Reply,
    #[serde(rename = "answer_status", default)]
    pub answer_status: i32,
    #[serde(rename = "tmp_external_userid", default)]
    pub tmp_external_user_id: String,
    #[serde(rename = "userid", default)]
    pub user_id: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Reply {
    #[serde(rename = "items", default)]
    pub items: Vec<crate::bean::oa::doc::wx_cp_form_answer::ReplyItem>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ReplyItem {
    #[serde(rename = "question_id", default)]
    pub question_id: i64,
    #[serde(rename = "text_reply", default)]
    pub text_reply: String,
    #[serde(rename = "option_reply", default)]
    pub option_reply: Vec<i32>,
    #[serde(rename = "option_extend_reply", default)]
    pub option_extend_reply: Vec<crate::bean::oa::doc::wx_cp_form_answer::OptionExtendReply>,
    #[serde(rename = "file_extend_reply", default)]
    pub file_extend_reply: Vec<crate::bean::oa::doc::wx_cp_form_answer::FileExtendReply>,
    #[serde(rename = "department_reply", default)]
    pub department_reply: crate::bean::oa::doc::wx_cp_form_answer::DepartmentReply,
    #[serde(rename = "member_reply", default)]
    pub member_reply: crate::bean::oa::doc::wx_cp_form_answer::MemberReply,
    #[serde(rename = "duration_reply", default)]
    pub duration_reply: crate::bean::oa::doc::wx_cp_form_answer::DurationReply,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OptionExtendReply {
    #[serde(rename = "option_reply", default)]
    pub option_reply: i32,
    #[serde(rename = "extend_text", default)]
    pub extend_text: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FileExtendReply {
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "fileid", default)]
    pub file_id: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DepartmentReply {
    #[serde(rename = "list", default)]
    pub list: Vec<crate::bean::oa::doc::wx_cp_form_answer::DepartmentItem>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DepartmentItem {
    #[serde(rename = "department_id", default)]
    pub department_id: i64,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MemberReply {
    #[serde(rename = "list", default)]
    pub list: Vec<crate::bean::oa::doc::wx_cp_form_answer::MemberItem>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MemberItem {
    #[serde(rename = "userid", default)]
    pub user_id: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DurationReply {
    #[serde(rename = "begin_time", default)]
    pub begin_time: i64,
    #[serde(rename = "end_time", default)]
    pub end_time: i64,
    #[serde(rename = "time_scale", default)]
    pub time_scale: i32,
    #[serde(rename = "day_range", default)]
    pub day_range: i32,
    #[serde(rename = "days", default)]
    pub days: f32,
    #[serde(rename = "hours", default)]
    pub hours: f32,
}

impl WxCpFormAnswer {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpFormAnswer 解析失败: {e}"))
    }
}

impl WxCpFormAnswer {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpFormAnswer 序列化失败: {e}"))
    }
}
