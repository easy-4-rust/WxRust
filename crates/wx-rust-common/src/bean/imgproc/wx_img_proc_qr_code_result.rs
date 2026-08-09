//! 对应 Java `me.chanjar.weixin.common.bean.imgproc.WxImgProcQrCodeResult`（由 gen_bean_structs.py 生成）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxImgProcQrCodeResult {
    /// imgSize
    #[serde(rename = "img_size", default)]
    pub img_size: ImgSize,
    /// codeResults
    #[serde(rename = "code_results", default)]
    pub code_results: Vec<CodeResults>,
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
pub struct CodeResults {
    /// typeName
    #[serde(rename = "type_name", default)]
    pub type_name: String,
    /// data
    #[serde(rename = "data", default)]
    pub data: String,
    /// pos
    #[serde(rename = "pos", default)]
    pub pos: Pos,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Pos {
    /// leftTop
    #[serde(rename = "left_top", default)]
    pub left_top: Coordinate,
    /// rightTop
    #[serde(rename = "right_top", default)]
    pub right_top: Coordinate,
    /// rightBottom
    #[serde(rename = "right_bottom", default)]
    pub right_bottom: Coordinate,
    /// leftBottom
    #[serde(rename = "left_bottom", default)]
    pub left_bottom: Coordinate,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Coordinate {
    /// x
    #[serde(rename = "x", default)]
    pub x: i32,
    /// y
    #[serde(rename = "y", default)]
    pub y: i32,
}
