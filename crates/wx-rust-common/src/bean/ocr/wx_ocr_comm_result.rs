//! 对应 Java `me.chanjar.weixin.common.bean.ocr.WxOcrCommResult`（由 gen_bean_structs.py 生成）。

use super::wx_ocr_img_size::WxOcrImgSize;
use super::wx_ocr_pos::WxOcrPos;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxOcrCommResult {
    /// imgSize
    #[serde(rename = "img_size", default)]
    pub img_size: WxOcrImgSize,
    /// items
    #[serde(rename = "items", default)]
    pub items: Vec<Items>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Items {
    /// text
    #[serde(rename = "text", default)]
    pub text: String,
    /// pos
    #[serde(rename = "pos", default)]
    pub pos: WxOcrPos,
}
