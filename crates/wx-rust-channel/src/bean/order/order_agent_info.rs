//! 对应 Java `me.chanjar.weixin.channel.bean.order.OrderAgentInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OrderAgentInfo {
    #[serde(rename = "agent_finder_id", default)]
    pub agent_finder_id: String,
    #[serde(rename = "agent_finder_nickname", default)]
    pub agent_finder_nickname: String,
}
