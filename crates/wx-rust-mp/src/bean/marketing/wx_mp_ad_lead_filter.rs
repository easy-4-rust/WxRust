//! 对应 Java `bean.marketing.WxMpAdLeadFilter`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpAdLeadFilter {
    #[serde(rename = "field", default)]
    pub field: String,
    #[serde(rename = "operator", default)]
    pub operator: String,
    #[serde(rename = "values", default)]
    pub values: Vec<String>,
}
