//! 对应 Java `me.chanjar.weixin.common.bean.ocr.WxOcrImgSize`（由 gen_bean_structs.py 生成）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxOcrImgSize {
    /// w
    #[serde(rename = "w", default)]
    pub w: i32,
    /// h
    #[serde(rename = "h", default)]
    pub h: i32,
}
