//! 对应 Java `me.chanjar.weixin.channel.bean.delivery.PackageAuditInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PackageAuditInfo {
    #[serde(rename = "item_name", default)]
    pub item_name: String,
    #[serde(rename = "item_value", default)]
    pub item_value: String,
}
