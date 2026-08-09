//! 对应 Java `bean.guide.WxMpGuideImgMaterialInfo`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpGuideImgMaterialInfo {
    #[serde(rename = "picurl", default)]
    pub pic_url: String,
}
