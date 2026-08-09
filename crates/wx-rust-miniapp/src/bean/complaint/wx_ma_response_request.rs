//! 对应 Java `cn.binarywang.wx.miniapp.bean.complaint.WxMaResponseRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaResponseRequest {
    #[serde(rename = "complaint_id", default)]
    pub complaint_id: String,
    #[serde(rename = "response_content", default)]
    pub response_content: String,
    #[serde(rename = "response_images", default)]
    pub response_images: Vec<String>,
    #[serde(rename = "jump_url", default)]
    pub jump_url: String,
    #[serde(rename = "jump_url_text", default)]
    pub jump_url_text: String,
}

impl WxMaResponseRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxMaResponseRequest 序列化失败: {e}"))
    }
}
