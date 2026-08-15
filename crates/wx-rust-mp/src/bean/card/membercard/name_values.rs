//! 对应 Java `bean.card.membercard.NameValues`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NameValues {
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "value", default)]
    pub value: String,
    #[serde(rename = "valueList", default)]
    pub value_list: Vec<String>,
}
