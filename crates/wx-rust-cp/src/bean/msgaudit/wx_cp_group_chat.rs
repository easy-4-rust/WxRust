//! 对应 Java `me.chanjar.weixin.cp.bean.msgaudit.WxCpGroupChat.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpGroupChat {
    #[serde(rename = "roomname", default)]
    pub room_name: String,
    #[serde(rename = "creator", default)]
    pub creator: String,
    #[serde(rename = "room_create_time", default)]
    pub room_create_time: i64,
    #[serde(rename = "notice", default)]
    pub notice: String,
    #[serde(rename = "members", default)]
    pub members: Vec<Member>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Member {
    #[serde(rename = "memberid", default)]
    pub member_id: String,
    #[serde(rename = "jointime", default)]
    pub join_time: i64,
}

impl WxCpGroupChat {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpGroupChat 解析失败: {e}"))
    }
}

impl WxCpGroupChat {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpGroupChat 序列化失败: {e}"))
    }
}
