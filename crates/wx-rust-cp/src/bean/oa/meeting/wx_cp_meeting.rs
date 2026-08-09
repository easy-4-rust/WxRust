//! 对应 Java `me.chanjar.weixin.cp.bean.oa.meeting.WxCpMeeting.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::oa::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpMeeting {
    #[serde(rename = "meetingid", default)]
    pub meeting_id: String,
    #[serde(rename = "admin_userid", default)]
    pub admin_user_id: String,
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "meeting_start", default)]
    pub meeting_start: i64,
    #[serde(rename = "meeting_duration", default)]
    pub meeting_duration: i32,
    #[serde(rename = "description", default)]
    pub description: String,
    #[serde(rename = "location", default)]
    pub location: String,
    #[serde(rename = "agentid", default)]
    pub agent_id: i32,
    #[serde(rename = "main_department", default)]
    pub main_department: i32,
    #[serde(rename = "meeting_type", default)]
    pub meeting_type: i32,
    #[serde(rename = "attendees", default)]
    pub attendees: Attendees,
    #[serde(rename = "cal_id", default)]
    pub cal_id: String,
    #[serde(rename = "settings", default)]
    pub settings: Setting,
    #[serde(rename = "reminders", default)]
    pub reminders: Reminder,
    #[serde(rename = "meeting_code", default)]
    pub meeting_code: String,
    #[serde(rename = "meeting_link", default)]
    pub meeting_link: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Attendees {
    #[serde(rename = "userid", default)]
    pub user_id: Vec<String>,
    #[serde(rename = "member", default)]
    pub member: Vec<Member>,
    #[serde(rename = "tmp_external_user", default)]
    pub tmp_external_user: Vec<TmpExternalUser>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Member {
    #[serde(rename = "userid", default)]
    pub userid: String,
    #[serde(rename = "status", default)]
    pub status: i32,
    #[serde(rename = "first_join_time", default)]
    pub first_join_time: i64,
    #[serde(rename = "last_quit_time", default)]
    pub last_quit_time: i64,
    #[serde(rename = "cumulative_time", default)]
    pub cumulative_time: i64,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TmpExternalUser {
    #[serde(rename = "tmp_external_userid", default)]
    pub tmp_external_userid: String,
    #[serde(rename = "first_join_time", default)]
    pub first_join_time: i64,
    #[serde(rename = "last_quit_time", default)]
    pub last_quit_time: i64,
    #[serde(rename = "total_join_count", default)]
    pub total_join_count: i32,
    #[serde(rename = "cumulative_time", default)]
    pub cumulative_time: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Reminder {
    #[serde(rename = "is_repeat", default)]
    pub is_repeat: i32,
    #[serde(rename = "repeat_type", default)]
    pub repeat_type: i32,
    #[serde(rename = "repeat_until", default)]
    pub repeat_until: i64,
    #[serde(rename = "repeat_interval", default)]
    pub repeat_interval: i32,
    #[serde(rename = "remind_before", default)]
    pub remind_before: Vec<i32>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Setting {
    #[serde(rename = "password", default)]
    pub password: String,
    #[serde(rename = "enable_waiting_room", default)]
    pub enable_waiting_room: bool,
    #[serde(rename = "allow_enter_before_host", default)]
    pub allow_enter_before_host: bool,
    #[serde(rename = "remind_scope", default)]
    pub remind_scope: i32,
    #[serde(rename = "enable_enter_mute", default)]
    pub enable_enter_mute: i32,
    #[serde(rename = "allow_external_user", default)]
    pub allow_external_user: bool,
    #[serde(rename = "enable_screen_watermark", default)]
    pub enable_screen_watermark: bool,
    #[serde(rename = "hosts", default)]
    pub hosts: crate::bean::oa::meeting::wx_cp_meeting::Attendees,
    #[serde(rename = "ring_users", default)]
    pub ring_users: crate::bean::oa::meeting::wx_cp_meeting::Attendees,
}

impl WxCpMeeting {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpMeeting 序列化失败: {e}"))
    }
}
