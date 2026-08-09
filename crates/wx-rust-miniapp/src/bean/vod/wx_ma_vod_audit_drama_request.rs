//! 对应 Java `cn.binarywang.wx.miniapp.bean.vod.WxMaVodAuditDramaRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaVodAuditDramaRequest {
    #[serde(rename = "drama_id", default)]
    pub drama_id: i32,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "media_count", default)]
    pub media_count: i64,
    #[serde(rename = "media_id_list", default)]
    pub media_id_list: Vec<i32>,
    #[serde(rename = "producer", default)]
    pub producer: String,
    #[serde(rename = "cover_material_id", default)]
    pub cover_material_id: String,
    #[serde(rename = "authorized_material_id", default)]
    pub authorized_material_id: String,
    #[serde(rename = "registration_number", default)]
    pub registration_number: String,
    #[serde(rename = "publish_license", default)]
    pub publish_license: String,
    #[serde(rename = "publish_license_material_id", default)]
    pub publish_license_material_id: String,
    #[serde(rename = "expedited", default)]
    pub expedited: i64,
}

impl WxMaVodAuditDramaRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxMaVodAuditDramaRequest 序列化失败: {e}"))
    }
}
