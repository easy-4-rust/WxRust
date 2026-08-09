//! 对应 Java `me.chanjar.weixin.channel.bean.order.AfterSaleDetail.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AfterSaleDetail {
    #[serde(rename = "on_aftersale_order_cnt", default)]
    pub on_after_sale_order_cnt: i32,
    #[serde(rename = "aftersale_order_list", default)]
    pub after_sale_order_list: Vec<AfterSaleOrderInfo>,
}
