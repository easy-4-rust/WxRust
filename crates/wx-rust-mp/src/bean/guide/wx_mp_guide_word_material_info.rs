//! 对应 Java `bean.guide.WxMpGuideWordMaterialInfo`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpGuideWordMaterialInfo {
    #[serde(rename = "word", default)]
    pub word: String,
    #[serde(rename = "create_time", default)]
    pub create_time: i64,
}
