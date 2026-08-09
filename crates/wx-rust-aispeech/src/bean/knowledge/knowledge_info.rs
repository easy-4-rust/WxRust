//! 对应 Java `me.chanjar.weixin.aispeech.bean.knowledge.KnowledgeInfo.java`。

use serde::{Deserialize, Serialize};

/// 知识条目信息。
///
/// 对应 Java `KnowledgeInfo`：知识库中单条知识的元数据与解析状态。
/// `metadata` 为 `Map<String, String>`（Java `Map`），`raw` 为原始
/// 响应结构（Java `JsonObject`）。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct KnowledgeInfo {
    /// 知识 id
    #[serde(default)]
    pub id: Option<String>,
    /// 租户 id（对应 Java `@SerializedName("tenant_id")`）
    #[serde(rename = "tenant_id", default)]
    pub tenant_id: Option<i64>,
    /// 知识库 id（对应 Java `@SerializedName("knowledge_base_id")`）
    #[serde(rename = "knowledge_base_id", default)]
    pub knowledge_base_id: Option<String>,
    /// 知识类型
    #[serde(default)]
    pub r#type: Option<String>,
    /// 标题
    #[serde(default)]
    pub title: Option<String>,
    /// 描述
    #[serde(default)]
    pub description: Option<String>,
    /// 来源
    #[serde(default)]
    pub source: Option<String>,
    /// 解析状态（对应 Java `@SerializedName("parse_status")`）
    #[serde(rename = "parse_status", default)]
    pub parse_status: Option<String>,
    /// 摘要状态（对应 Java `@SerializedName("summary_status")`）
    #[serde(rename = "summary_status", default)]
    pub summary_status: Option<String>,
    /// 启用状态（对应 Java `@SerializedName("enable_status")`）
    #[serde(rename = "enable_status", default)]
    pub enable_status: Option<String>,
    /// 元数据键值对（对应 Java `Map<String, String> metadata`）
    #[serde(default)]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// 创建时间（对应 Java `@SerializedName("created_at")`）
    #[serde(rename = "created_at", default)]
    pub created_at: Option<String>,
    /// 更新时间（对应 Java `@SerializedName("updated_at")`）
    #[serde(rename = "updated_at", default)]
    pub updated_at: Option<String>,
    /// 原始结构（对应 Java `JsonObject raw`）
    #[serde(default)]
    pub raw: Option<serde_json::Value>,
}

impl KnowledgeInfo {
    /// 从 JSON 构建（对应 Java `WxGsonBuilder.fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("KnowledgeInfo 解析失败: {e}"))
    }
}
