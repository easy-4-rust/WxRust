//! 对应 Java `me.chanjar.weixin.channel.bean.order.OrderCommissionInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OrderCommissionInfo {
    #[serde(rename = "sku_id", default)]
    pub sku_id: String,
    #[serde(rename = "nickname", default)]
    pub nickname: String,
    #[serde(rename = "type", default)]
    pub r#type: i32,
    #[serde(rename = "status", default)]
    pub status: i32,
    #[serde(rename = "amount", default)]
    pub amount: i32,
    #[serde(rename = "finder_id", default)]
    pub finder_id: String,
    #[serde(rename = "openfinderid", default)]
    pub open_finder_id: String,
    #[serde(rename = "talent_id", default)]
    pub talent_id: String,
}
