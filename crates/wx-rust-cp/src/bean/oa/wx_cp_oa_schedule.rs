//! 对应 Java `me.chanjar.weixin.cp.bean.oa.WxCpOaSchedule.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpOaSchedule {
    #[serde(rename = "schedule_id", default)]
    pub schedule_id: String,
    #[serde(rename = "sequence", default)]
    pub sequence: i32,
    #[serde(rename = "organizer", default)]
    pub organizer: String,
    #[serde(rename = "admins", default)]
    pub admins: Vec<String>,
    #[serde(rename = "attendees", default)]
    pub attendees: Vec<Attendee>,
    #[serde(rename = "summary", default)]
    pub summary: String,
    #[serde(rename = "description", default)]
    pub description: String,
    #[serde(rename = "reminders", default)]
    pub reminders: Reminder,
    #[serde(rename = "location", default)]
    pub location: String,
    #[serde(rename = "start_time", default)]
    pub start_time: i64,
    #[serde(rename = "end_time", default)]
    pub end_time: i64,
    #[serde(rename = "status", default)]
    pub status: i32,
    #[serde(rename = "cal_id", default)]
    pub cal_id: String,
    #[serde(rename = "is_whole_day", default)]
    pub is_whole_day: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Attendee {
    #[serde(rename = "userid", default)]
    pub userid: String,
    #[serde(rename = "response_status", default)]
    pub response_status: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Reminder {
    #[serde(rename = "is_remind", default)]
    pub is_remind: i32,
    #[serde(rename = "is_repeat", default)]
    pub is_repeat: i32,
    #[serde(rename = "remind_before_event_secs", default)]
    pub remind_before_event_secs: i32,
    #[serde(rename = "remind_time_diffs", default)]
    pub remind_time_diffs: Vec<i32>,
    #[serde(rename = "repeat_type", default)]
    pub repeat_type: i32,
    #[serde(rename = "repeat_until", default)]
    pub repeat_until: i64,
    #[serde(rename = "is_custom_repeat", default)]
    pub is_custom_repeat: i32,
    #[serde(rename = "repeat_interval", default)]
    pub repeat_interval: i32,
    #[serde(rename = "repeat_day_of_week", default)]
    pub repeat_day_of_week: Vec<i32>,
    #[serde(rename = "repeat_day_of_month", default)]
    pub repeat_day_of_month: Vec<i32>,
    #[serde(rename = "timezone", default)]
    pub timezone: i32,
    #[serde(rename = "exclude_time_list", default)]
    pub exclude_time_list: Vec<ExcludeTime>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExcludeTime {
    #[serde(rename = "start_time", default)]
    pub start_time: i64,
}

impl WxCpOaSchedule {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpOaSchedule 序列化失败: {e}"))
    }
}
