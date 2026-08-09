//! 对应 Java `me.chanjar.weixin.cp.bean.oa.mail.WxCpMailScheduleSendRequest.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::oa::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpMailScheduleSendRequest {
    #[serde(rename = "to", default)]
    pub to: TO,
    #[serde(rename = "cc", default)]
    pub cc: CC,
    #[serde(rename = "bcc", default)]
    pub bcc: BCC,
    #[serde(rename = "subject", default)]
    pub subject: String,
    #[serde(rename = "content", default)]
    pub content: String,
    #[serde(rename = "attachment_list", default)]
    pub attachment_list: Vec<Attachment>,
    #[serde(rename = "content_type", default)]
    pub content_type: String,
    #[serde(rename = "enable_id_trans", default)]
    pub enable_id_trans: i32,
    #[serde(rename = "schedule", default)]
    pub schedule: Schedule,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Schedule {
    #[serde(rename = "is_remind", default)]
    pub schedule_id: String,
    #[serde(rename = "method", default)]
    pub method: String,
    #[serde(rename = "location", default)]
    pub location: String,
    #[serde(rename = "start_time", default)]
    pub start_time: i32,
    #[serde(rename = "end_time", default)]
    pub end_time: i32,
    #[serde(rename = "reminders", default)]
    pub reminders: crate::bean::oa::mail::wx_cp_mail_schedule_send_request::Reminders,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Reminders {
    #[serde(rename = "is_remind", default)]
    pub is_remind: i32,
    #[serde(rename = "remind_before_event_mins", default)]
    pub remind_before_event_mins: i32,
    #[serde(rename = "is_repeat", default)]
    pub is_repeat: i32,
    #[serde(rename = "is_custom_repeat", default)]
    pub is_custom_repeat: i32,
    #[serde(rename = "timezone", default)]
    pub time_zone: i32,
    #[serde(rename = "repeat_interval", default)]
    pub repeat_interval: i32,
    #[serde(rename = "repeat_type", default)]
    pub repeat_type: i32,
    #[serde(rename = "repeat_day_of_week", default)]
    pub repeat_day_of_week: Vec<i32>,
    #[serde(rename = "repeat_day_of_month", default)]
    pub repeat_day_of_month: Vec<String>,
    #[serde(rename = "repeat_week_of_month", default)]
    pub repeat_week_of_month: Vec<String>,
    #[serde(rename = "repeat_month_of_year", default)]
    pub repeat_month_of_year: Vec<String>,
    #[serde(rename = "repeat_until", default)]
    pub repeat_until: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TO {
    #[serde(rename = "emails", default)]
    pub emails: Vec<String>,
    #[serde(rename = "userids", default)]
    pub user_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CC {
    #[serde(rename = "emails", default)]
    pub emails: Vec<String>,
    #[serde(rename = "userids", default)]
    pub user_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BCC {
    #[serde(rename = "emails", default)]
    pub emails: Vec<String>,
    #[serde(rename = "userids", default)]
    pub user_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Attachment {
    #[serde(rename = "file_name", default)]
    pub file_name: String,
    #[serde(rename = "content", default)]
    pub content: String,
}

impl WxCpMailScheduleSendRequest {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpMailScheduleSendRequest 解析失败: {e}"))
    }
}

impl WxCpMailScheduleSendRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxCpMailScheduleSendRequest 序列化失败: {e}"))
    }
}
