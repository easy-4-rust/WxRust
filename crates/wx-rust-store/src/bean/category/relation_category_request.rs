//! 对应 Java `com.binarywang.wxjava.store.bean.category.RelationCategoryRequest.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 RelationCategoryRequest；对应 Java com.binarywang.wxjava.store.bean.category.RelationCategoryRequest.java。
pub struct RelationCategoryRequest {
    #[serde(rename = "is_filter_status", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_filter_status: Option<bool>,
    #[serde(rename = "status", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}
