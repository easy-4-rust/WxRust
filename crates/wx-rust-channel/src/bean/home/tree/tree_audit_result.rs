//! 对应 Java `me.chanjar.weixin.channel.bean.home.tree.TreeAuditResult.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::home::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TreeAuditResult {
    #[serde(rename = "version", default)]
    pub version: i32,
    #[serde(rename = "audit_results", default)]
    pub audit_results: Vec<TreeAuditResultDetail>,
}
