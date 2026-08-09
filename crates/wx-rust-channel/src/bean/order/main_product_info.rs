//! 对应 Java `me.chanjar.weixin.channel.bean.order.MainProductInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MainProductInfo {
    #[serde(rename = "gift_cnt", default)]
    pub gift_cnt: i32,
    #[serde(rename = "task_id", default)]
    pub task_id: i32,
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    #[serde(rename = "sku_id", default)]
    pub sku_id: i32,
}
