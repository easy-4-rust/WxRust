//! 对应 Java `com.binarywang.wxjava.store.bean.audit.CategoryAuditRequest.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::CategoryAuditInfo;

/// 微信小店 CategoryAuditRequest 数据类型；对应 Java com.binarywang.wxjava.store.bean.audit.CategoryAuditRequest.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CategoryAuditRequest {
    #[serde(rename = "category_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_info: Option<CategoryAuditInfo>,
}
