//! 对应 Java `me.chanjar.weixin.cp.bean.living.WxCpLivingModifyRequest.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpLivingModifyRequest {
    #[serde(rename = "livingid", default)]
    pub living_id: String,
    #[serde(rename = "theme", default)]
    pub theme: String,
    #[serde(rename = "living_start", default)]
    pub living_start: i64,
    #[serde(rename = "living_duration", default)]
    pub living_duration: i64,
    #[serde(rename = "remind_time", default)]
    pub remind_time: i64,
    #[serde(rename = "description", default)]
    pub description: String,
    #[serde(rename = "type", default)]
    pub r#type: i32,
}

impl WxCpLivingModifyRequest {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpLivingModifyRequest 解析失败: {e}"))
    }
}

impl WxCpLivingModifyRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpLivingModifyRequest 序列化失败: {e}"))
    }
}
