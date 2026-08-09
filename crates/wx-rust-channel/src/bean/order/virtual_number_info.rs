//! 对应 Java `me.chanjar.weixin.channel.bean.order.VirtualNumberInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct VirtualNumberInfo {
    #[serde(rename = "virtual_number", default)]
    pub virtual_number: String,
    #[serde(rename = "extension", default)]
    pub extension: String,
    #[serde(rename = "expiration", default)]
    pub expiration: i64,
}
