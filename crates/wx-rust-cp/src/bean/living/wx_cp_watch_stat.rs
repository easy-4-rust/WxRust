//! 对应 Java `me.chanjar.weixin.cp.bean.living.WxCpWatchStat.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpWatchStat {
    #[serde(rename = "ending", default)]
    pub ending: i32,
    #[serde(rename = "next_key", default)]
    pub next_key: String,
    #[serde(rename = "stat_info", default)]
    pub stat_info: StatInfo,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StatInfo {
    #[serde(rename = "users", default)]
    pub users: Vec<crate::bean::living::wx_cp_watch_stat::User>,
    #[serde(rename = "external_users", default)]
    pub external_users: Vec<crate::bean::living::wx_cp_watch_stat::ExternalUser>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct User {
    #[serde(rename = "userid", default)]
    pub userid: String,
    #[serde(rename = "watch_time", default)]
    pub watch_time: i64,
    #[serde(rename = "is_comment", default)]
    pub is_comment: i32,
    #[serde(rename = "is_mic", default)]
    pub is_mic: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExternalUser {
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "type", default)]
    pub r#type: i32,
    #[serde(rename = "external_userid", default)]
    pub external_userid: String,
    #[serde(rename = "watch_time", default)]
    pub watch_time: i64,
    #[serde(rename = "is_comment", default)]
    pub is_comment: i32,
    #[serde(rename = "is_mic", default)]
    pub is_mic: i32,
}

impl WxCpWatchStat {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpWatchStat 解析失败: {e}"))
    }
}

impl WxCpWatchStat {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpWatchStat 序列化失败: {e}"))
    }
}
