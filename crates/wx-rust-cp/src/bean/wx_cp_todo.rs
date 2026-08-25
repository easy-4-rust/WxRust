//! 企业微信待办信息 bean。
//!
//! 对应 Java `me.chanjar.weixin.cp.bean.todo.WxCpTodo`。
//!
//! 官方文档：
//! - 获取待办详情：<https://developer.work.weixin.qq.com/document/path/101524>
//! - 更新待办状态：<https://developer.work.weixin.qq.com/document/path/101534>

use serde::{Deserialize, Serialize};

/// 待办参与人。
///
/// 对应 Java: `WxCpTodo.Attendee`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WxCpTodoAttendee {
    /// 待办参与人 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub userid: Option<String>,
    /// 参与人的待办状态：0 - 完成，1 - 进行中。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

/// 待办提醒。
///
/// 对应 Java: `WxCpTodo.Reminder`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WxCpTodoReminder {
    /// 提醒时间戳（整型秒数）。
    #[serde(rename = "remind_time", skip_serializing_if = "Option::is_none")]
    pub remind_time: Option<i64>,
}

/// 待办信息。
///
/// 对应 Java: `WxCpTodo`
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WxCpTodo {
    /// 待办 ID。
    #[serde(rename = "todo_id", skip_serializing_if = "Option::is_none")]
    pub todo_id: Option<String>,
    /// 待办内容。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 待办创建人 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,
    /// 待办状态：0 - 已完成，1 - 进行中，2 - 已删除。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// 待办创建时间戳（整型秒数）。
    #[serde(rename = "create_time", skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    /// 待办参与人列表。
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attendees: Vec<WxCpTodoAttendee>,
    /// 待办截止时间戳（整型秒数）。
    #[serde(rename = "end_time", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// 提醒列表。
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reminders: Vec<WxCpTodoReminder>,
}

impl WxCpTodo {
    /// 从 JSON 字符串解析（对应 Java `WxCpGsonBuilder.fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpTodo 解析失败: {e}"))
    }

    /// 序列化为 JSON 字符串（对应 Java `toJson`）。
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serde_roundtrip() {
        let todo = WxCpTodo {
            todo_id: Some("TODO_123".to_string()),
            content: Some("完成季度报告".to_string()),
            creator: Some("zhangsan".to_string()),
            status: Some(1),
            create_time: Some(1700000000),
            attendees: vec![
                WxCpTodoAttendee {
                    userid: Some("zhangsan".to_string()),
                    status: Some(1),
                },
                WxCpTodoAttendee {
                    userid: Some("lisi".to_string()),
                    status: Some(0),
                },
            ],
            end_time: Some(1700100000),
            reminders: vec![WxCpTodoReminder {
                remind_time: Some(1700050000),
            }],
        };
        let json = todo.to_json();
        let parsed = WxCpTodo::from_json(&json).unwrap();
        assert_eq!(todo, parsed);
    }

    #[test]
    fn parse_from_api_response() {
        let json = r#"{
            "todo_id": "TODO_456",
            "content": "review PR",
            "creator": "wangwu",
            "status": 0,
            "create_time": 1700000000,
            "attendees": [{"userid": "wangwu", "status": 0}],
            "end_time": 1700100000,
            "reminders": []
        }"#;
        let todo = WxCpTodo::from_json(json).unwrap();
        assert_eq!(todo.todo_id.as_deref(), Some("TODO_456"));
        assert_eq!(todo.status, Some(0));
        assert_eq!(todo.attendees.len(), 1);
        assert_eq!(todo.attendees[0].userid.as_deref(), Some("wangwu"));
    }

    #[test]
    fn parse_minimal() {
        let json = r#"{"todo_id":"T1"}"#;
        let todo = WxCpTodo::from_json(json).unwrap();
        assert_eq!(todo.todo_id.as_deref(), Some("T1"));
        assert!(todo.attendees.is_empty());
    }
}
