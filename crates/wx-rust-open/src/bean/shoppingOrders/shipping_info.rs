//! 对应 Java `me.chanjar.weixin.open.bean.shoppingOrders.ShippingInfo.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ShippingInfo {
    #[serde(rename = "order_key", default)]
    pub order_key: OrderKeyBean,
    #[serde(rename = "delivery_mode", default)]
    pub delivery_mode: i32,
    #[serde(rename = "shipping_list", default)]
    pub shipping_list: Vec<ShippingListBean>,
    #[serde(rename = "upload_time", default)]
    pub upload_time: String,
}
