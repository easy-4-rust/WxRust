//! 对应 Java `me.chanjar.weixin.cp.bean.living.WxCpLivingInfo.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpLivingInfo {
    #[serde(rename = "theme", default)]
    pub theme: String,
    #[serde(rename = "living_start", default)]
    pub living_start: i64,
    #[serde(rename = "living_duration", default)]
    pub living_duration: i64,
    #[serde(rename = "status", default)]
    pub status: i32,
    #[serde(rename = "reserve_living_duration", default)]
    pub reserve_living_duration: i64,
    #[serde(rename = "reserve_start", default)]
    pub reserve_start: i64,
    #[serde(rename = "description", default)]
    pub description: String,
    #[serde(rename = "anchor_userid", default)]
    pub anchor_userid: String,
    #[serde(rename = "main_department", default)]
    pub main_department: i64,
    #[serde(rename = "viewer_num", default)]
    pub viewer_num: i32,
    #[serde(rename = "comment_num", default)]
    pub comment_num: i32,
    #[serde(rename = "mic_num", default)]
    pub mic_num: i32,
    #[serde(rename = "open_replay", default)]
    pub open_replay: i32,
    #[serde(rename = "replay_status", default)]
    pub replay_status: i32,
    #[serde(rename = "type", default)]
    pub r#type: i32,
    #[serde(rename = "push_stream_url", default)]
    pub push_stream_url: String,
    #[serde(rename = "online_count", default)]
    pub online_count: i32,
    #[serde(rename = "subscribe_count", default)]
    pub subscribe_count: i32,
}

impl WxCpLivingInfo {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpLivingInfo 解析失败: {e}"))
    }
}

impl WxCpLivingInfo {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpLivingInfo 序列化失败: {e}"))
    }
}
