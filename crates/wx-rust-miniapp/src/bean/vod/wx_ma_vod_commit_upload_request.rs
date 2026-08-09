//! 对应 Java `cn.binarywang.wx.miniapp.bean.vod.WxMaVodCommitUploadRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaVodCommitUploadRequest {
    #[serde(rename = "upload_id", default)]
    pub upload_id: String,
    #[serde(rename = "media_part_infos", default)]
    pub media_part_infos: Vec<PartInfo>,
    #[serde(rename = "cover_part_infos", default)]
    pub cover_part_infos: Vec<PartInfo>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PartInfo {
    #[serde(rename = "part_number", default)]
    pub part_number: i32,
    #[serde(rename = "etag", default)]
    pub etag: String,
}

impl WxMaVodCommitUploadRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxMaVodCommitUploadRequest 序列化失败: {e}"))
    }
}
