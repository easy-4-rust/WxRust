//! 对应 Java `com.binarywang.wxjava.store.bean.category.RelationCategoryItem.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 RelationCategoryItem；对应 Java com.binarywang.wxjava.store.bean.category.RelationCategoryItem.java。
pub struct RelationCategoryItem {
    #[serde(rename = "id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "status", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "uneffective_reason", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uneffective_reason: Option<String>,
    #[serde(rename = "effective_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<i64>,
    #[serde(rename = "uneffective_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uneffective_time: Option<i64>,
    #[serde(rename = "qua_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qua_id: Option<i64>,
}
