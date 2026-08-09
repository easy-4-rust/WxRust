//! 对应 Java `me.chanjar.weixin.common.bean.ocr.WxOcrPos`（由 gen_bean_structs.py 生成）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxOcrPos {
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
