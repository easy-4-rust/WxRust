//! 对应 Java `me.chanjar.weixin.common.bean.imgproc.WxImgProcAiCropResult`（由 gen_bean_structs.py 生成）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxImgProcAiCropResult {
    /// imgSize
    #[serde(rename = "img_size", default)]
    pub img_size: ImgSize,
    /// results
    #[serde(rename = "results", default)]
    pub results: Vec<Results>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ImgSize {
    /// w
    #[serde(rename = "w", default)]
    pub w: i32,
    /// h
    #[serde(rename = "h", default)]
    pub h: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Results {
    /// cropLeft
    #[serde(rename = "crop_left", default)]
    pub crop_left: i32,
    /// cropTop
    #[serde(rename = "crop_top", default)]
    pub crop_top: i32,
    /// cropRight
    #[serde(rename = "crop_right", default)]
    pub crop_right: i32,
    /// cropBottom
    #[serde(rename = "crop_bottom", default)]
    pub crop_bottom: i32,
}
