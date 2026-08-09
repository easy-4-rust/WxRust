//! 对应 Java `me.chanjar.weixin.aispeech.bean.knowledge.KnowledgeMoveRequest.java`。

use serde::{Deserialize, Serialize};

/// 知识迁移请求。
///
/// 对应 Java `KnowledgeMoveRequest`：`moveKnowledge` 的入参，将一批知识从
/// 源知识库迁移到目标知识库。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct KnowledgeMoveRequest {
    /// 待迁移的知识 id 列表（对应 Java `@SerializedName("knowledge_ids")`）
    #[serde(rename = "knowledge_ids", default)]
    pub knowledge_ids: Option<Vec<String>>,
    /// 源知识库 id（对应 Java `@SerializedName("source_kb_id")` 字段
    /// `sourceKnowledgeBaseId`）
    #[serde(rename = "source_kb_id", default)]
    pub source_knowledge_base_id: Option<String>,
    /// 目标知识库 id（对应 Java `@SerializedName("target_kb_id")` 字段
    /// `targetKnowledgeBaseId`）
    #[serde(rename = "target_kb_id", default)]
    pub target_knowledge_base_id: Option<String>,
    /// 迁移模式（如 `reuse_vectors`）
    #[serde(default)]
    pub mode: Option<String>,
}
