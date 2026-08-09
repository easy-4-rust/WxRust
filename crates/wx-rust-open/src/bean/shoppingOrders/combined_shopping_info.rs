//! 对应 Java `me.chanjar.weixin.open.bean.shoppingOrders.CombinedShoppingInfo.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CombinedShoppingInfo {
    #[serde(rename = "order_key", default)]
    pub order_key: OrderKeyBean,
    #[serde(rename = "sub_orders", default)]
    pub sub_orders: Vec<SubOrderListBean>,
    #[serde(rename = "payer", default)]
    pub payer: PayerBean,
    #[serde(rename = "upload_time", default)]
    pub upload_time: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SubOrderListBean {
    #[serde(rename = "order_key", default)]
    pub order_key: OrderKeyBean,
    #[serde(rename = "merchant_order_no", default)]
    pub merchant_order_no: String,
    #[serde(rename = "order_detail_jump_link", default)]
    pub order_detail_jump_link: OrderDetailBean,
    #[serde(rename = "item_list", default)]
    pub item_list: Vec<OrderItemListBean>,
    #[serde(rename = "logistics_type", default)]
    pub logistics_type: i32,
}
