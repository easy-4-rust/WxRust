//! 对应 Java `me.chanjar.weixin.channel.bean.delivery.FreshInspectParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FreshInspectParam {
    #[serde(rename = "order_id", default)]
    pub order_id: String,
    #[serde(rename = "audit_items", default)]
    pub audit_items: Vec<PackageAuditInfo>,
}
