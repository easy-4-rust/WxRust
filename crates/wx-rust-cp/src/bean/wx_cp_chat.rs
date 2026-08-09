//! 企业微信群聊。
//!
//! 对应 Java `me.chanjar.weixin.cp.bean.WxCpChat`，线格式以
//! `util/json/WxCpChatGsonAdapter` 为准：`chatid`/`name`/`owner`/
//! `userlist`；null 省略，`userlist` 非空才输出。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpChat {
    /// 群聊 id（wire `chatid`）。
    #[serde(rename = "chatid", skip_serializing_if = "Option::is_none", default)]
    pub id: Option<String>,
    /// 群聊名。
    #[serde(rename = "name", skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
    /// 群主 userid。
    #[serde(rename = "owner", skip_serializing_if = "Option::is_none", default)]
    pub owner: Option<String>,
    /// 群成员 userid 列表（wire `userlist`）。
    #[serde(rename = "userlist", skip_serializing_if = "Option::is_none", default)]
    pub users: Option<Vec<String>>,
}

impl WxCpChat {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpChat 解析失败: {e}"))
    }

    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpChat 序列化失败: {e}"))
    }
}
