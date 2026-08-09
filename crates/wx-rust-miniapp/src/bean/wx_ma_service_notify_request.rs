//! 对应 Java `cn.binarywang.wx.miniapp.bean.WxMaServiceNotifyRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaServiceNotifyRequest {
    #[serde(rename = "openid", default)]
    pub openid: String,
    #[serde(rename = "notify_type", default)]
    pub notify_type: i32,
    #[serde(rename = "notify_code", default)]
    pub notify_code: String,
    #[serde(rename = "content_json", default)]
    pub content_json: String,
    #[serde(rename = "check_json", default)]
    pub check_json: String,
}

impl WxMaServiceNotifyRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxMaServiceNotifyRequest 序列化失败: {e}"))
    }
}
