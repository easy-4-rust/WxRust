//! 对应 Java `me.chanjar.weixin.cp.bean.intelligentrobot.WxCpIntelligentRobotSendMessageResponse.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpIntelligentRobotSendMessageResponse {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "msg_id", default)]
    pub msg_id: String,
    #[serde(rename = "session_id", default)]
    pub session_id: String,
}

impl WxCpIntelligentRobotSendMessageResponse {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json)
            .map_err(|e| format!("WxCpIntelligentRobotSendMessageResponse 解析失败: {e}"))
    }
}

impl WxCpIntelligentRobotSendMessageResponse {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxCpIntelligentRobotSendMessageResponse 序列化失败: {e}"))
    }
}
