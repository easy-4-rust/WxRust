//! 对应 Java `cn.binarywang.wx.miniapp.bean.vod.WxMaVodMediaInfo.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaVodMediaInfo {
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "description", default)]
    pub description: String,
    #[serde(rename = "media_id", default)]
    pub media_id: i32,
    #[serde(rename = "drama_id", default)]
    pub drama_id: i32,
    #[serde(rename = "create_time", default)]
    pub create_time: i64,
    #[serde(rename = "file_size", default)]
    pub file_size: String,
    #[serde(rename = "duration", default)]
    pub duration: i64,
    #[serde(rename = "expire_time", default)]
    pub expire_time: i64,
    #[serde(rename = "cover_url", default)]
    pub cover_url: String,
    #[serde(rename = "original_url", default)]
    pub original_url: String,
    #[serde(rename = "mp4_url", default)]
    pub mp4_url: String,
    #[serde(rename = "hls_url", default)]
    pub hls_url: String,
    #[serde(rename = "audit_detail", default)]
    pub audit_detail: MediaAuditDetail,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MediaAuditDetail {
    #[serde(rename = "status", default)]
    pub status: i32,
    #[serde(rename = "create_time", default)]
    pub create_time: i64,
    #[serde(rename = "audit_time", default)]
    pub audit_time: i64,
    #[serde(rename = "reason", default)]
    pub reason: String,
    #[serde(rename = "evidence_material_id_list", default)]
    pub evidence_material_id_list: Vec<String>,
}
