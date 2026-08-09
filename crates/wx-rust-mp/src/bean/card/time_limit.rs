//! 对应 Java `bean.card.TimeLimit`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TimeLimit {
    #[serde(rename = "type", default)]
    pub r#type: String,
    #[serde(rename = "begin_hour", default)]
    pub begin_hour: i32,
    #[serde(rename = "begin_minute", default)]
    pub begin_minute: i32,
    #[serde(rename = "end_hour", default)]
    pub end_hour: i32,
    #[serde(rename = "end_minute", default)]
    pub end_minute: i32,
}
