//! 对应 Java `me.chanjar.weixin.channel.bean.league.ExpressInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExpressInfo {
    #[serde(rename = "send_time", default)]
    pub send_time: String,
    #[serde(rename = "address_info", default)]
    pub address_info: AddressInfo,
    #[serde(rename = "shipping_method", default)]
    pub shipping_method: String,
}
