//! 对应 Java `cn.binarywang.wx.miniapp.bean.vod.WxMaVodMediaPlaybackInfo.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaVodMediaPlaybackInfo {
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "description", default)]
    pub description: String,
    #[serde(rename = "media_id", default)]
    pub media_id: i32,
    #[serde(rename = "duration", default)]
    pub duration: i64,
    #[serde(rename = "cover_url", default)]
    pub cover_url: String,
    #[serde(rename = "mp4_url", default)]
    pub mp4_url: String,
    #[serde(rename = "hls_url", default)]
    pub hls_url: String,
}
