//! 对应 Java `bean.material.WxMpMaterialCountResult`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpMaterialCountResult {
    #[serde(rename = "voiceCount", default)]
    pub voice_count: i32,
    #[serde(rename = "videoCount", default)]
    pub video_count: i32,
    #[serde(rename = "imageCount", default)]
    pub image_count: i32,
    #[serde(rename = "newsCount", default)]
    pub news_count: i32,
}

impl WxMpMaterialCountResult {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxMpMaterialCountResult 解析失败: {e}"))
    }
}
