//! 对应 Java `cn.binarywang.wx.miniapp.bean.vod.WxMaVodGetMediaLinkRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaVodGetMediaLinkRequest {
    #[serde(rename = "media_id", default)]
    pub media_id: i32,
    #[serde(rename = "t", default)]
    pub t: i64,
    #[serde(rename = "expr", default)]
    pub expr: i64,
    #[serde(rename = "rlimit", default)]
    pub rlimit: i64,
    #[serde(rename = "us", default)]
    pub us: String,
    #[serde(rename = "whref", default)]
    pub wh_ref: String,
    #[serde(rename = "bkref", default)]
    pub bk_ref: String,
}

impl WxMaVodGetMediaLinkRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxMaVodGetMediaLinkRequest 序列化失败: {e}"))
    }
}
