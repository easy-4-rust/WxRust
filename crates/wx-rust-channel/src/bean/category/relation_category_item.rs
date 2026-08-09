//! 对应 Java `me.chanjar.weixin.channel.bean.category.RelationCategoryItem.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RelationCategoryItem {
    #[serde(rename = "id", default)]
    pub id: i64,
    #[serde(rename = "status", default)]
    pub status: i32,
    #[serde(rename = "uneffective_reason", default)]
    pub uneffective_reason: String,
    #[serde(rename = "effective_time", default)]
    pub effective_time: i64,
    #[serde(rename = "uneffective_time", default)]
    pub uneffective_time: i64,
    #[serde(rename = "qua_id", default)]
    pub qua_id: i64,
}
