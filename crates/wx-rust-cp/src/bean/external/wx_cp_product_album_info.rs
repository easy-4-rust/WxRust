//! 对应 Java `me.chanjar.weixin.cp.bean.external.WxCpProductAlbumInfo.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpProductAlbumInfo {
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    #[serde(rename = "product_sn", default)]
    pub product_sn: String,
    #[serde(rename = "description", default)]
    pub description: String,
    #[serde(rename = "price", default)]
    pub price: i32,
    #[serde(rename = "create_time", default)]
    pub create_time: i64,
    #[serde(rename = "attachments", default)]
    pub attachments: Vec<crate::bean::oa::mail::wx_cp_mail_common_send_request::Attachment>,
}

impl WxCpProductAlbumInfo {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpProductAlbumInfo 解析失败: {e}"))
    }
}

impl WxCpProductAlbumInfo {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpProductAlbumInfo 序列化失败: {e}"))
    }
}
