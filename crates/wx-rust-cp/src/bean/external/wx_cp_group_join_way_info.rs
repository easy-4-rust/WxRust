//! 对应 Java `me.chanjar.weixin.cp.bean.external.WxCpGroupJoinWayInfo.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpGroupJoinWayInfo {
    #[serde(rename = "join_way", default)]
    pub join_way: JoinWay,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct JoinWay {
    #[serde(rename = "config_id", default)]
    pub config_id: String,
    #[serde(rename = "scene", default)]
    pub scene: i32,
    #[serde(rename = "remark", default)]
    pub remark: String,
    #[serde(rename = "auto_create_room", default)]
    pub auto_create_room: i32,
    #[serde(rename = "room_base_name", default)]
    pub room_base_name: String,
    #[serde(rename = "room_base_id", default)]
    pub room_base_id: i32,
    #[serde(rename = "chat_id_list", default)]
    pub chat_id_list: Vec<String>,
    #[serde(rename = "qr_code", default)]
    pub qr_code: String,
    #[serde(rename = "state", default)]
    pub state: String,
}

impl WxCpGroupJoinWayInfo {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpGroupJoinWayInfo 解析失败: {e}"))
    }
}

impl WxCpGroupJoinWayInfo {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpGroupJoinWayInfo 序列化失败: {e}"))
    }
}
