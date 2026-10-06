//! 对应 Java `com.binarywang.wxjava.store.bean.order.OrderAgentInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 OrderAgentInfo；对应 Java com.binarywang.wxjava.store.bean.order.OrderAgentInfo.java。
pub struct OrderAgentInfo {
    #[serde(rename = "agent_finder_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_finder_id: Option<String>,
    #[serde(rename = "agent_finder_nickname", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_finder_nickname: Option<String>,
}
