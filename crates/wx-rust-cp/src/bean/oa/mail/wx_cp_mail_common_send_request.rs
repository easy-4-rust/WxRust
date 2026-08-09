//! 对应 Java `me.chanjar.weixin.cp.bean.oa.mail.WxCpMailCommonSendRequest.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::oa::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpMailCommonSendRequest {
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

impl WxCpMailCommonSendRequest {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpMailCommonSendRequest 解析失败: {e}"))
    }
}

impl WxCpMailCommonSendRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxCpMailCommonSendRequest 序列化失败: {e}"))
    }
}
