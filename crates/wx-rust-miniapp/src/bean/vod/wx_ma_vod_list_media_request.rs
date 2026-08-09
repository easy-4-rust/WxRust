//! 对应 Java `cn.binarywang.wx.miniapp.bean.vod.WxMaVodListMediaRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaVodListMediaRequest {
    #[serde(rename = "drama_id", default)]
    pub drama_id: i32,
    #[serde(rename = "media_name", default)]
    pub media_name: String,
    #[serde(rename = "media_name_fuzzy", default)]
    pub media_name_fuzzy: String,
    #[serde(rename = "start_time", default)]
    pub start_time: i64,
    #[serde(rename = "end_time", default)]
    pub end_time: i64,
    #[serde(rename = "offset", default)]
    pub offset: i32,
    #[serde(rename = "limit", default)]
    pub limit: i32,
}

impl WxMaVodListMediaRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxMaVodListMediaRequest 序列化失败: {e}"))
    }
}
