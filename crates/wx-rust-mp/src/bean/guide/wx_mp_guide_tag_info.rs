//! 对应 Java `bean.guide.WxMpGuideTagInfo`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpGuideTagInfo {
    #[serde(rename = "tag_name", default)]
    pub tag_name: String,
    #[serde(rename = "tag_values", default)]
    pub values: Vec<String>,
}
