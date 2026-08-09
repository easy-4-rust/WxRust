//! 对应 Java `me.chanjar.weixin.cp.bean.intelligentrobot.WxCpIntelligentRobotMessage.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpIntelligentRobotMessage {
    #[serde(rename = "msgid", default)]
    pub msg_id: String,
    #[serde(rename = "aibotid", default)]
    pub ai_bot_id: String,
    #[serde(rename = "chatid", default)]
    pub chat_id: String,
    #[serde(rename = "chattype", default)]
    pub chat_type: String,
    #[serde(rename = "from", default)]
    pub from: WxCpIntelligentRobotMessageFrom,
    #[serde(rename = "response_url", default)]
    pub response_url: String,
    #[serde(rename = "msgtype", default)]
    pub msg_type: String,
    #[serde(rename = "text", default)]
    pub text: Text,
    #[serde(rename = "image", default)]
    pub image: Image,
    #[serde(rename = "mixed", default)]
    pub mixed: Mixed,
    #[serde(rename = "voice", default)]
    pub voice: Voice,
    #[serde(rename = "file", default)]
    pub file: FileInfo,
    #[serde(rename = "video", default)]
    pub video: Video,
    #[serde(rename = "quote", default)]
    pub quote: Quote,
    #[serde(rename = "stream", default)]
    pub stream: Stream,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpIntelligentRobotMessageFrom {
    #[serde(rename = "userid", default)]
    pub userid: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Text {
    #[serde(rename = "content", default)]
    pub content: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Image {
    #[serde(rename = "url", default)]
    pub url: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Voice {
    #[serde(rename = "content", default)]
    pub content: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FileInfo {
    #[serde(rename = "url", default)]
    pub url: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Video {
    #[serde(rename = "url", default)]
    pub url: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Stream {
    #[serde(rename = "id", default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Mixed {
    #[serde(rename = "msg_item", default)]
    pub msg_item: Vec<crate::bean::intelligentrobot::wx_cp_intelligent_robot_message::MixedItem>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MixedItem {
    #[serde(rename = "msgtype", default)]
    pub msg_type: String,
    #[serde(rename = "text", default)]
    pub text: crate::bean::intelligentrobot::wx_cp_intelligent_robot_message::Text,
    #[serde(rename = "image", default)]
    pub image: crate::bean::intelligentrobot::wx_cp_intelligent_robot_message::Image,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Quote {
    #[serde(rename = "msgtype", default)]
    pub msg_type: String,
    #[serde(rename = "text", default)]
    pub text: crate::bean::intelligentrobot::wx_cp_intelligent_robot_message::Text,
    #[serde(rename = "image", default)]
    pub image: crate::bean::intelligentrobot::wx_cp_intelligent_robot_message::Image,
    #[serde(rename = "mixed", default)]
    pub mixed: crate::bean::intelligentrobot::wx_cp_intelligent_robot_message::Mixed,
    #[serde(rename = "voice", default)]
    pub voice: crate::bean::intelligentrobot::wx_cp_intelligent_robot_message::Voice,
    #[serde(rename = "file", default)]
    pub file: crate::bean::intelligentrobot::wx_cp_intelligent_robot_message::FileInfo,
    #[serde(rename = "video", default)]
    pub video: crate::bean::intelligentrobot::wx_cp_intelligent_robot_message::Video,
}

impl WxCpIntelligentRobotMessage {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpIntelligentRobotMessage 解析失败: {e}"))
    }
}

impl WxCpIntelligentRobotMessage {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxCpIntelligentRobotMessage 序列化失败: {e}"))
    }
}
