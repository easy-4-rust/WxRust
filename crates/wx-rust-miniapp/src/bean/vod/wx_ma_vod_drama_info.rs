//! 对应 Java `cn.binarywang.wx.miniapp.bean.vod.WxMaVodDramaInfo.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaVodDramaInfo {
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "producer", default)]
    pub producer: String,
    #[serde(rename = "playwright", default)]
    pub playwright: String,
    #[serde(rename = "drama_id", default)]
    pub drama_id: i32,
    #[serde(rename = "create_time", default)]
    pub create_time: i64,
    #[serde(rename = "cover_url", default)]
    pub cover_url: String,
    #[serde(rename = "media_count", default)]
    pub media_count: i64,
    #[serde(rename = "expedited", default)]
    pub expedited: i64,
    #[serde(rename = "production_license", default)]
    pub production_license: String,
    #[serde(rename = "description", default)]
    pub description: String,
    #[serde(rename = "status", default)]
    pub status: String,
    #[serde(rename = "audit_detail", default)]
    pub audit_detail: DramaAuditDetail,
    #[serde(rename = "media_list", default)]
    pub media_list: Vec<DramaMediaInfo>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DramaAuditDetail {
    #[serde(rename = "status", default)]
    pub status: i32,
    #[serde(rename = "create_time", default)]
    pub create_time: i64,
    #[serde(rename = "audit_time", default)]
    pub audit_time: i64,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DramaMediaInfo {
    #[serde(rename = "media_id", default)]
    pub media_id: i32,
}
