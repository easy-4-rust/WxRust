//! 对应 Java `com.binarywang.wxjava.store.bean.order.AfterSaleDetail.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::AfterSaleOrderInfo;

/// 微信小店 AfterSaleDetail 数据类型；对应 Java com.binarywang.wxjava.store.bean.order.AfterSaleDetail.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AfterSaleDetail {
    #[serde(rename = "on_aftersale_order_cnt", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_after_sale_order_cnt: Option<i32>,
    #[serde(rename = "aftersale_order_list", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_sale_order_list: Option<Vec<AfterSaleOrderInfo>>,
}
