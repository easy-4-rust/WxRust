//! 对应 Java `me.chanjar.weixin.cp.bean.external.WxCpUserExternalGroupChatList.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpUserExternalGroupChatList {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "group_chat_list", default)]
    pub group_chat_list: Vec<ChatStatus>,
    #[serde(rename = "next_cursor", default)]
    pub next_cursor: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ChatStatus {
    #[serde(rename = "chat_id", default)]
    pub chat_id: String,
    #[serde(rename = "status", default)]
    pub status: i32,
}

impl WxCpUserExternalGroupChatList {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json)
            .map_err(|e| format!("WxCpUserExternalGroupChatList 解析失败: {e}"))
    }
}
