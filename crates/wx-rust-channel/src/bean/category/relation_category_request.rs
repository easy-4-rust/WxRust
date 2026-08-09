//! 对应 Java `me.chanjar.weixin.channel.bean.category.RelationCategoryRequest.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RelationCategoryRequest {
    #[serde(rename = "is_filter_status", default)]
    pub is_filter_status: bool,
    #[serde(rename = "status", default)]
    pub status: i32,
}
