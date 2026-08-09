//! 对应 Java `me.chanjar.weixin.cp.bean.oa.mail.WxCpMailMeetingSendRequest.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::oa::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpMailMeetingSendRequest {
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
    #[serde(rename = "meeting", default)]
    pub meeting: Meeting,
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

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Meeting {
    #[serde(rename = "option", default)]
    pub option:
        crate::bean::oa::mail::wx_cp_mail_meeting_send_request::WxCpMailMeetingSendRequestOption,
    #[serde(rename = "hosts", default)]
    pub hosts: crate::bean::oa::mail::wx_cp_mail_meeting_send_request::Hosts,
    #[serde(rename = "meeting_admins", default)]
    pub meeting_admins: crate::bean::oa::mail::wx_cp_mail_meeting_send_request::MeetingAdmins,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpMailMeetingSendRequestOption {
    #[serde(rename = "password", default)]
    pub password: String,
    #[serde(rename = "auto_record", default)]
    pub auto_record: i32,
    #[serde(rename = "enable_waiting_room", default)]
    pub enable_waiting_room: bool,
    #[serde(rename = "allow_enter_before_host", default)]
    pub allow_enter_before_host: bool,
    #[serde(rename = "enter_restraint", default)]
    pub enter_restraint: i32,
    #[serde(rename = "enable_screen_watermark", default)]
    pub enable_screen_watermark: bool,
    #[serde(rename = "enable_enter_mute", default)]
    pub enable_enter_mute: i32,
    #[serde(rename = "remind_scope", default)]
    pub remind_scope: i32,
    #[serde(rename = "water_mark_type", default)]
    pub water_mark_type: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Hosts {
    #[serde(rename = "userids", default)]
    pub user_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MeetingAdmins {
    #[serde(rename = "userids", default)]
    pub user_ids: Vec<String>,
}

impl WxCpMailMeetingSendRequest {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpMailMeetingSendRequest 解析失败: {e}"))
    }
}

impl WxCpMailMeetingSendRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxCpMailMeetingSendRequest 序列化失败: {e}"))
    }
}
