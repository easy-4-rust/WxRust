//! 对应 Java `me.chanjar.weixin.common.bean.imgproc.WxImgProcSuperResolutionResult`（由 gen_bean_structs.py 生成）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxImgProcSuperResolutionResult {
    /// mediaId
    #[serde(rename = "media_id", default)]
    pub media_id: String,
}
