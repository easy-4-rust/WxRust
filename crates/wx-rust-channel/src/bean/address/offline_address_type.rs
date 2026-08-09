//! 对应 Java `me.chanjar.weixin.channel.bean.address.OfflineAddressType.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OfflineAddressType {
    #[serde(rename = "same_city", default)]
    pub same_city: i32,
    #[serde(rename = "pickup", default)]
    pub pickup: i32,
}
