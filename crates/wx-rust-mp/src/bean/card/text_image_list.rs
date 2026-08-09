//! 对应 Java `bean.card.TextImageList`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TextImageList {
    #[serde(rename = "image_url", default)]
    pub image_url: String,
    #[serde(rename = "text", default)]
    pub text: String,
}
