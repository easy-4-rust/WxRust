//! 对应 Java `me.chanjar.weixin.cp.bean.intelligentrobot.WxCpIntelligentRobotCreateRequest.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpIntelligentRobotCreateRequest {
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "description", default)]
    pub description: String,
    #[serde(rename = "avatar", default)]
    pub avatar: String,
}

impl WxCpIntelligentRobotCreateRequest {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json)
            .map_err(|e| format!("WxCpIntelligentRobotCreateRequest 解析失败: {e}"))
    }
}

impl WxCpIntelligentRobotCreateRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxCpIntelligentRobotCreateRequest 序列化失败: {e}"))
    }
}
