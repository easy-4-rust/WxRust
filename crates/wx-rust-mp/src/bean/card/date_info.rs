//! 对应 Java `bean.card.DateInfo`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DateInfo {
    #[serde(rename = "type", default)]
    pub r#type: String,
    #[serde(rename = "begin_timestamp", default)]
    pub begin_timestamp: i64,
    #[serde(rename = "end_timestamp", default)]
    pub end_timestamp: i64,
    #[serde(rename = "fixed_term", default)]
    pub fixed_term: i32,
    #[serde(rename = "fixed_begin_term", default)]
    pub fixed_begin_term: i32,
}
