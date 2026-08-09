//! 对应 Java `cn.binarywang.wx.miniapp.bean.kefu.WxMaKfSessionList.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaKfSessionList {
    #[serde(rename = "sessionlist", default)]
    pub session_list: Vec<SessionInfo>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SessionInfo {
    #[serde(rename = "openid", default)]
    pub openid: String,
    #[serde(rename = "createtime", default)]
    pub create_time: i64,
    #[serde(rename = "latest_time", default)]
    pub latest_time: i64,
}

impl WxMaKfSessionList {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxMaKfSessionList 解析失败: {e}"))
    }
}

impl WxMaKfSessionList {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxMaKfSessionList 序列化失败: {e}"))
    }
}
