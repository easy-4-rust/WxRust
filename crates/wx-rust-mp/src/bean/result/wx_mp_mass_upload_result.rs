//! 群发素材上传结果。
//!
//! 对应 Java `me.chanjar.weixin.mp.bean.result.WxMpMassUploadResult`。
//! 线格式由 `WxMpMassUploadResultAdapter` 决定：`type`/`media_id`/`created_at`。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpMassUploadResult {
    #[serde(rename = "type", default)]
    pub r#type: String,
    #[serde(rename = "media_id", default)]
    pub media_id: String,
    #[serde(rename = "created_at", default)]
    pub created_at: i64,
}

impl WxMpMassUploadResult {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("素材上传结果解析失败: {e}"))
    }
}
