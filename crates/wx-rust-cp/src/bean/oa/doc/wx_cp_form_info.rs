//! 对应 Java `me.chanjar.weixin.cp.bean.oa.doc.WxCpFormInfo.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::oa::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpFormInfo {
    #[serde(rename = "formid", default)]
    pub form_id: String,
    #[serde(rename = "form_title", default)]
    pub form_title: String,
    #[serde(rename = "form_desc", default)]
    pub form_desc: String,
    #[serde(rename = "form_header", default)]
    pub form_header: String,
    #[serde(rename = "form_question", default)]
    pub form_question: FormQuestion,
    #[serde(rename = "form_setting", default)]
    pub form_setting: FormSetting,
    #[serde(rename = "repeated_id", default)]
    pub repeated_id: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FormQuestion {
    #[serde(rename = "items", default)]
    pub items: Vec<crate::bean::oa::doc::wx_cp_form_info::QuestionItem>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct QuestionItem {
    #[serde(rename = "question_id", default)]
    pub question_id: i64,
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "pos", default)]
    pub pos: i32,
    #[serde(rename = "status", default)]
    pub status: i32,
    #[serde(rename = "reply_type", default)]
    pub reply_type: i32,
    #[serde(rename = "must_reply", default)]
    pub must_reply: bool,
    #[serde(rename = "note", default)]
    pub note: String,
    #[serde(rename = "option_item", default)]
    pub option_item: Vec<crate::bean::oa::doc::wx_cp_form_info::OptionItem>,
    #[serde(rename = "placeholder", default)]
    pub placeholder: String,
    #[serde(rename = "question_extend_setting", default)]
    pub question_extend_setting: serde_json::Value,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OptionItem {
    #[serde(rename = "key", default)]
    pub key: i32,
    #[serde(rename = "value", default)]
    pub value: String,
    #[serde(rename = "status", default)]
    pub status: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FormSetting {
    #[serde(rename = "fill_out_auth", default)]
    pub fill_out_auth: i32,
    #[serde(rename = "fill_in_range", default)]
    pub fill_in_range: crate::bean::oa::doc::wx_cp_form_info::FillInRange,
    #[serde(rename = "setting_manager_range", default)]
    pub setting_manager_range: crate::bean::oa::doc::wx_cp_form_info::SettingManagerRange,
    #[serde(rename = "timed_repeat_info", default)]
    pub timed_repeat_info: crate::bean::oa::doc::wx_cp_form_info::TimedRepeatInfo,
    #[serde(rename = "allow_multi_fill", default)]
    pub allow_multi_fill: bool,
    #[serde(rename = "max_fill_cnt", default)]
    pub max_fill_cnt: i32,
    #[serde(rename = "timed_finish", default)]
    pub timed_finish: i64,
    #[serde(rename = "can_anonymous", default)]
    pub can_anonymous: bool,
    #[serde(rename = "can_notify_submit", default)]
    pub can_notify_submit: bool,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FillInRange {
    #[serde(rename = "userids", default)]
    pub user_ids: Vec<String>,
    #[serde(rename = "departmentids", default)]
    pub department_ids: Vec<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SettingManagerRange {
    #[serde(rename = "userids", default)]
    pub user_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TimedRepeatInfo {
    #[serde(rename = "enable", default)]
    pub enable: bool,
    #[serde(rename = "week_flag", default)]
    pub week_flag: i32,
    #[serde(rename = "remind_time", default)]
    pub remind_time: i64,
    #[serde(rename = "repeat_type", default)]
    pub repeat_type: i32,
    #[serde(rename = "skip_holiday", default)]
    pub skip_holiday: bool,
    #[serde(rename = "day_of_month", default)]
    pub day_of_month: i32,
    #[serde(rename = "fork_finish_type", default)]
    pub fork_finish_type: i32,
    #[serde(rename = "rule_ctime", default)]
    pub rule_ctime: i64,
    #[serde(rename = "rule_mtime", default)]
    pub rule_mtime: i64,
}
