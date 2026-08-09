//! 对应 Java `me.chanjar.weixin.common.bean.result.WxMinishopPicFileCustomizeResult`（由 gen_bean_structs.py 生成）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMinishopPicFileCustomizeResult {
    /// mediaId
    #[serde(rename = "mediaId", default)]
    pub media_id: String,
    /// tempImgUrl
    #[serde(rename = "tempImgUrl", default)]
    pub temp_img_url: String,
}
