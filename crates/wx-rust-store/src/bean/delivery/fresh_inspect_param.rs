//! 对应 Java `com.binarywang.wxjava.store.bean.delivery.FreshInspectParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::PackageAuditInfo;

/// 微信小店 FreshInspectParam 数据类型；对应 Java com.binarywang.wxjava.store.bean.delivery.FreshInspectParam.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FreshInspectParam {
    #[serde(rename = "order_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(rename = "audit_items", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_items: Option<Vec<PackageAuditInfo>>,
}
