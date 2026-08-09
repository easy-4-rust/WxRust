//! 对应 Java `me.chanjar.weixin.aispeech.bean.knowledge.KnowledgeManualCreateRequest.java`。

use serde::{Deserialize, Serialize};

/// 手工创建/更新知识的请求。
///
/// 对应 Java `KnowledgeManualCreateRequest`：`createKnowledgeByManual` 与
/// `updateManualKnowledge` 的入参。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct KnowledgeManualCreateRequest {
    /// 内容（Markdown 文本）
    #[serde(default)]
    pub content: Option<String>,
    /// 标题
    #[serde(default)]
    pub title: Option<String>,
    /// 描述
    #[serde(default)]
    pub description: Option<String>,
    /// 状态
    #[serde(default)]
    pub status: Option<String>,
}
