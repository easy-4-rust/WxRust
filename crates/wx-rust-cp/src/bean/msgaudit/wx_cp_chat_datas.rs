//! 对应 Java `me.chanjar.weixin.cp.bean.msgaudit.WxCpChatDatas.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpChatDatas {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "sdk", default)]
    pub sdk: i64,
    #[serde(rename = "chatdata", default)]
    pub chat_data: Vec<WxCpChatData>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpChatData {
    #[serde(rename = "seq", default)]
    pub seq: i64,
    #[serde(rename = "msgid", default)]
    pub msg_id: String,
    #[serde(rename = "publickey_ver", default)]
    pub publickey_ver: i32,
    #[serde(rename = "encrypt_random_key", default)]
    pub encrypt_random_key: String,
    #[serde(rename = "encrypt_chat_msg", default)]
    pub encrypt_chat_msg: String,
}

impl WxCpChatDatas {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpChatDatas 解析失败: {e}"))
    }
}

impl WxCpChatDatas {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpChatDatas 序列化失败: {e}"))
    }
}
