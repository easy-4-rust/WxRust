//! 对应 Java `bean.draft.WxMpDraftImageInfo`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpDraftImageInfo {
    #[serde(rename = "image_list", default)]
    pub image_list: Vec<ImageItem>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ImageItem {
    #[serde(rename = "image_media_id", default)]
    pub image_media_id: String,
}
