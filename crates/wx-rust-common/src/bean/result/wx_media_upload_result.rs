//! 对应 Java `me.chanjar.weixin.common.bean.result.WxMediaUploadResult`（由 gen_bean_structs.py 生成）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMediaUploadResult {
    /// url
    #[serde(rename = "url", default)]
    pub url: Option<String>,
    /// type
    #[serde(rename = "type", default)]
    pub r#type: Option<String>,
    /// mediaId
    #[serde(
        rename(serialize = "mediaId", deserialize = "media_id"),
        alias = "mediaId",
        default
    )]
    pub media_id: Option<String>,
    /// thumbMediaId
    #[serde(
        rename(serialize = "thumbMediaId", deserialize = "thumb_media_id"),
        alias = "thumbMediaId",
        default
    )]
    pub thumb_media_id: Option<String>,
    /// createdAt
    #[serde(
        rename(serialize = "createdAt", deserialize = "created_at"),
        alias = "createdAt",
        default
    )]
    pub created_at: i64,
}
