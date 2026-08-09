//! 对应 Java `me.chanjar.weixin.aispeech.bean.knowledge.KnowledgeListResult.java`。

use serde::{Deserialize, Serialize};

/// 知识列表响应。
///
/// 对应 Java `KnowledgeListResult`：`listKnowledge` 的响应，`data` 为
/// 知识条目列表（`getData()` 显式暴露）。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct KnowledgeListResult {
    /// 知识条目列表
    #[serde(default)]
    pub data: Option<Vec<crate::bean::knowledge::KnowledgeInfo>>,
    /// 页码
    #[serde(default)]
    pub page: Option<i32>,
    /// 每页条数（对应 Java `@SerializedName("page_size")`）
    #[serde(rename = "page_size", default)]
    pub page_size: Option<i32>,
    /// 总条数
    #[serde(default)]
    pub total: Option<i32>,
    /// 是否成功
    #[serde(default)]
    pub success: Option<bool>,
}
