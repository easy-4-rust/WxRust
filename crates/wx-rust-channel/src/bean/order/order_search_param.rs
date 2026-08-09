//! 对应 Java `me.chanjar.weixin.channel.bean.order.OrderSearchParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[allow(unused_imports)]
use crate::bean::base::StreamPageParam;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OrderSearchParam {
    #[serde(rename = "page_size", default)]
    pub page_size: i32,
    #[serde(rename = "next_key", default)]
    pub next_key: String,
    #[serde(rename = "search_condition", default)]
    pub search_condition: OrderSearchCondition,
    #[serde(rename = "on_aftersale_order_exist", default)]
    pub on_after_sale_order_exist: i32,
    #[serde(rename = "status", default)]
    pub status: i32,
}
