//! 对应 Java `me.chanjar.weixin.cp.bean.external.contact.WxCpGroupMsgListResult.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::external::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpGroupMsgListResult {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "group_msg_list", default)]
    pub group_msg_list: Vec<ExternalContactGroupMsgInfo>,
    #[serde(rename = "next_cursor", default)]
    pub next_cursor: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExternalContactGroupMsgInfo {
    #[serde(rename = "msgid", default)]
    pub msg_id: String,
    #[serde(rename = "creator", default)]
    pub creator: String,
    #[serde(rename = "text", default)]
    pub text: crate::bean::wx_cp_user_external_contact_info::Text,
    #[serde(rename = "attachments", default)]
    pub attachments: Vec<crate::bean::oa::mail::wx_cp_mail_common_send_request::Attachment>,
    #[serde(rename = "create_type", default)]
    pub create_type: i32,
    #[serde(rename = "create_time", default)]
    pub create_time: i64,
}

impl WxCpGroupMsgListResult {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpGroupMsgListResult 解析失败: {e}"))
    }
}
