//! 对应 Java `me.chanjar.weixin.cp.bean.external.WxCpMsgTemplate.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpMsgTemplate {
    #[serde(rename = "chat_type", default)]
    pub chat_type: String,
    #[serde(rename = "external_userid", default)]
    pub external_userid: Vec<String>,
    #[serde(rename = "chat_id_list", default)]
    pub chat_id_list: Vec<String>,
    #[serde(rename = "tag_filter", default)]
    pub tag_filter: crate::bean::external::msg::tag_filter::TagFilter,
    #[serde(rename = "sender", default)]
    pub sender: String,
    #[serde(rename = "allow_select", default)]
    pub allow_select: bool,
    #[serde(rename = "text", default)]
    pub text: crate::bean::wx_cp_user_external_contact_info::Text,
    #[serde(rename = "attachments", default)]
    pub attachments: Vec<crate::bean::oa::mail::wx_cp_mail_common_send_request::Attachment>,
}

impl WxCpMsgTemplate {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpMsgTemplate 解析失败: {e}"))
    }
}

impl WxCpMsgTemplate {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpMsgTemplate 序列化失败: {e}"))
    }
}
