//! 对应 Java `me.chanjar.weixin.aispeech.bean.knowledge.KnowledgeUrlCreateRequest.java`。

use serde::{Deserialize, Serialize};

/// 通过 URL 创建知识的请求。
///
/// 对应 Java `KnowledgeUrlCreateRequest`：`createKnowledgeByUrl` 的入参。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct KnowledgeUrlCreateRequest {
    /// 网页地址
    #[serde(default)]
    pub url: Option<String>,
    /// 标题
    #[serde(default)]
    pub title: Option<String>,
    /// 描述
    #[serde(default)]
    pub description: Option<String>,
}
