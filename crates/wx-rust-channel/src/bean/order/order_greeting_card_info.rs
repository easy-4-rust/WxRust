//! 对应 Java `me.chanjar.weixin.channel.bean.order.OrderGreetingCardInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OrderGreetingCardInfo {
    #[serde(rename = "giver_name", default)]
    pub giver_name: String,
    #[serde(rename = "receiver_name", default)]
    pub receiver_name: String,
    #[serde(rename = "greeting_message", default)]
    pub greeting_message: String,
}
