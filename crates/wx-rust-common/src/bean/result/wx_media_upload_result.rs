//! 对应 Java `me.chanjar.weixin.common.bean.result.WxMediaUploadResult`（由 gen_bean_structs.py 生成）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMediaUploadResult {
    /// url
    #[serde(rename = "url", default)]
    pub url: String,
    /// type
    #[serde(rename = "type", default)]
    pub r#type: String,
    /// mediaId
    #[serde(rename = "mediaId", default)]
    pub media_id: String,
    /// thumbMediaId
    #[serde(rename = "thumbMediaId", default)]
    pub thumb_media_id: String,
    /// createdAt
    #[serde(rename = "createdAt", default)]
    pub created_at: i64,
}
