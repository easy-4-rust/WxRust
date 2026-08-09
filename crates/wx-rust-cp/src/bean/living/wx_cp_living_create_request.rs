//! 对应 Java `me.chanjar.weixin.cp.bean.living.WxCpLivingCreateRequest.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpLivingCreateRequest {
    #[serde(rename = "anchor_userid", default)]
    pub anchor_userid: String,
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
    #[serde(rename = "agentid", default)]
    pub agent_id: i32,
    #[serde(rename = "activity_cover_mediaid", default)]
    pub activity_cover_mediaid: String,
    #[serde(rename = "activity_share_mediaid", default)]
    pub activity_share_mediaid: String,
    #[serde(rename = "activity_detail", default)]
    pub activity_detail: ActivityDetail,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ActivityDetail {
    #[serde(rename = "image_list", default)]
    pub image_list: Vec<String>,
    #[serde(rename = "description", default)]
    pub description: String,
}

impl WxCpLivingCreateRequest {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpLivingCreateRequest 解析失败: {e}"))
    }
}

impl WxCpLivingCreateRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpLivingCreateRequest 序列化失败: {e}"))
    }
}
