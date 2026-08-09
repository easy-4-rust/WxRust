//! 对应 Java `cn.binarywang.wx.miniapp.bean.device.WxMaDeviceSubscribeMessageRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaDeviceSubscribeMessageRequest {
    #[serde(rename = "to_openid_list", default)]
    pub to_openid_list: Vec<String>,
    #[serde(rename = "sn", default)]
    pub sn: String,
    #[serde(rename = "template_id", default)]
    pub template_id: String,
    #[serde(rename = "page", default)]
    pub page: String,
    #[serde(rename = "miniprogram_state", default)]
    pub miniprogram_state: String,
    #[serde(rename = "modelId", default)]
    pub model_id: String,
    #[serde(rename = "lang", default)]
    pub lang: String,
    #[serde(rename = "data", default)]
    pub data: serde_json::Value,
}

impl WxMaDeviceSubscribeMessageRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxMaDeviceSubscribeMessageRequest 序列化失败: {e}"))
    }
}
