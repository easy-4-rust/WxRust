//! 对应 Java `bean.draft.WxMpDraftCoverInfo`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpDraftCoverInfo {
    #[serde(rename = "crop_percent_list", default)]
    pub crop_percent_list: Vec<CropPercent>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CropPercent {
    #[serde(rename = "ratio", default)]
    pub ratio: String,
    #[serde(rename = "x1", default)]
    pub x1: String,
    #[serde(rename = "y1", default)]
    pub y1: String,
    #[serde(rename = "x2", default)]
    pub x2: String,
    #[serde(rename = "y2", default)]
    pub y2: String,
}
