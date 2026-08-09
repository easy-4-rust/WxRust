//! 对应 Java `me.chanjar.weixin.aispeech.bean.knowledge.KnowledgeTagRequest.java`。

use serde::{Deserialize, Serialize};

/// 知识标签创建/更新请求。
///
/// 对应 Java `KnowledgeTagRequest`：`createKnowledgeBaseTag` 与
/// `updateKnowledgeBaseTag` 的入参。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct KnowledgeTagRequest {
    /// 标签名称
    #[serde(default)]
    pub name: Option<String>,
    /// 标签颜色
    #[serde(default)]
    pub color: Option<String>,
    /// 排序值（对应 Java `@SerializedName("sort_order")`）
    #[serde(rename = "sort_order", default)]
    pub sort_order: Option<i32>,
}
