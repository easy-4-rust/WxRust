//! 对应 Java `cn.binarywang.wx.miniapp.bean.xpay.WxMaXPayUploadVpFileRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaXPayUploadVpFileRequest {
    #[serde(rename = "env", default)]
    pub env: i32,
    #[serde(rename = "base64_img", default)]
    pub base64_img: String,
    #[serde(rename = "img_url", default)]
    pub img_url: String,
    #[serde(rename = "file_name", default)]
    pub file_name: String,
}

impl WxMaXPayUploadVpFileRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxMaXPayUploadVpFileRequest 序列化失败: {e}"))
    }
}
