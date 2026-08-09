//! 对应 Java `me.chanjar.weixin.aispeech.bean.knowledge.KnowledgeUpdateRequest.java`。

use serde::{Deserialize, Serialize};

/// 知识更新请求。
///
/// 对应 Java `KnowledgeUpdateRequest`：`updateKnowledge` 的入参。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct KnowledgeUpdateRequest {
    /// 标题
    #[serde(default)]
    pub title: Option<String>,
    /// 描述
    #[serde(default)]
    pub description: Option<String>,
    /// 启用状态（对应 Java `@SerializedName("enable_status")`）
    #[serde(rename = "enable_status", default)]
    pub enable_status: Option<String>,
}
