//! 对应 Java `me.chanjar.weixin.cp.bean.message.WxCpMessageSendStatistics.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpMessageSendStatistics {
    #[serde(rename = "statistics", default)]
    pub statistics: Vec<StatisticItem>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StatisticItem {
    #[serde(rename = "app_name", default)]
    pub app_name: String,
    #[serde(rename = "agentid", default)]
    pub agent_id: i32,
    #[serde(rename = "count", default)]
    pub count: i32,
}

impl WxCpMessageSendStatistics {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpMessageSendStatistics 解析失败: {e}"))
    }
}
