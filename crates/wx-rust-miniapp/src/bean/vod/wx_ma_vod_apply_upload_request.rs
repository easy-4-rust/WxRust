//! 对应 Java `cn.binarywang.wx.miniapp.bean.vod.WxMaVodApplyUploadRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaVodApplyUploadRequest {
    #[serde(rename = "media_name", default)]
    pub media_name: String,
    #[serde(rename = "media_type", default)]
    pub media_type: String,
    #[serde(rename = "cover_type", default)]
    pub cover_type: String,
    #[serde(rename = "source_context", default)]
    pub source_context: String,
}

impl WxMaVodApplyUploadRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxMaVodApplyUploadRequest 序列化失败: {e}"))
    }
}
