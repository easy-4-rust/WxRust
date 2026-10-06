//! 对应 Java `com.binarywang.wxjava.store.bean.home.tree.TreeAuditResult.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::TreeAuditResultDetail;

/// 微信小店 TreeAuditResult 数据类型；对应 Java com.binarywang.wxjava.store.bean.home.tree.TreeAuditResult.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TreeAuditResult {
    #[serde(rename = "version", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    #[serde(rename = "audit_results", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_results: Option<Vec<TreeAuditResultDetail>>,
}
