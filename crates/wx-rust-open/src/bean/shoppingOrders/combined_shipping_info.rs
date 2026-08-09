//! 对应 Java `me.chanjar.weixin.open.bean.shoppingOrders.CombinedShippingInfo.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CombinedShippingInfo {
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
    #[serde(rename = "delivery_mode", default)]
    pub delivery_mode: i32,
    #[serde(rename = "shipping_list", default)]
    pub shipping_list: Vec<ShippingListBean>,
}
