//! 对应 Java `me.chanjar.weixin.channel.bean.product.TimingOnSaleInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TimingOnSaleInfo {
    #[serde(rename = "status", default)]
    pub status: i32,
    #[serde(rename = "onsale_time", default)]
    pub on_sale_time: i64,
    #[serde(rename = "is_hide_price", default)]
    pub is_hide_price: i32,
    #[serde(rename = "task_id", default)]
    pub task_id: i32,
}
