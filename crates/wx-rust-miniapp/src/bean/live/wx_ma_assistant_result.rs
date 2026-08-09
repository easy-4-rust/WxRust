//! 对应 Java `cn.binarywang.wx.miniapp.bean.live.WxMaAssistantResult.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaAssistantResult {
    #[serde(rename = "count", default)]
    pub count: i32,
    #[serde(rename = "maxCount", default)]
    pub max_count: i32,
    #[serde(rename = "errcode", default)]
    pub errcode: i32,
    #[serde(rename = "list", default)]
    pub list: Vec<Assistant>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Assistant {
    #[serde(rename = "timestamp", default)]
    pub timestamp: i64,
    #[serde(rename = "headimg", default)]
    pub headimg: String,
    #[serde(rename = "nickname", default)]
    pub nickname: String,
    #[serde(rename = "alias", default)]
    pub alias: String,
    #[serde(rename = "openid", default)]
    pub openid: String,
}

impl WxMaAssistantResult {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxMaAssistantResult 解析失败: {e}"))
    }
}
