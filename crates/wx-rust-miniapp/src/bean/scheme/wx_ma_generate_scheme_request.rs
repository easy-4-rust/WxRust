//! 对应 Java `cn.binarywang.wx.miniapp.bean.scheme.WxMaGenerateSchemeRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaGenerateSchemeRequest {
    #[serde(rename = "jump_wxa", default)]
    pub jump_wxa: JumpWxa,
    #[serde(rename = "is_expire", default)]
    pub is_expire: bool,
    #[serde(rename = "expire_time", default)]
    pub expire_time: i64,
    #[serde(rename = "expire_type", default)]
    pub expire_type: i32,
    #[serde(rename = "expire_interval", default)]
    pub expire_interval: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct JumpWxa {
    #[serde(rename = "path", default)]
    pub path: String,
    #[serde(rename = "query", default)]
    pub query: String,
    #[serde(rename = "env_version", default)]
    pub env_version: String,
}

impl WxMaGenerateSchemeRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxMaGenerateSchemeRequest 序列化失败: {e}"))
    }
}
