//! 对应 Java `bean.card.WxMpCardCodeCheckcodeResult`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpCardCodeCheckcodeResult {
    #[serde(rename = "exist_code", default)]
    pub exist_code: Vec<String>,
    #[serde(rename = "not_exist_code", default)]
    pub not_exist_code: Vec<String>,
}
