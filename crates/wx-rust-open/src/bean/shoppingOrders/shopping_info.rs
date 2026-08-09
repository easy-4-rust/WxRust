//! 对应 Java `me.chanjar.weixin.open.bean.shoppingOrders.ShoppingInfo.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ShoppingInfo {
    #[serde(rename = "order_key", default)]
    pub order_key: OrderKeyBean,
    #[serde(rename = "order_list", default)]
    pub order_list: Vec<OrderListBean>,
    #[serde(rename = "payer", default)]
    pub payer: PayerBean,
    #[serde(rename = "logistics_type", default)]
    pub logistics_type: i32,
    #[serde(rename = "upload_time", default)]
    pub upload_time: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OrderListBean {
    #[serde(rename = "merchant_order_no", default)]
    pub merchant_order_no: String,
    #[serde(rename = "order_detail_jump_link", default)]
    pub order_detail_jump_link: OrderDetailBean,
    #[serde(rename = "item_list", default)]
    pub item_list: Vec<OrderItemListBean>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OrderDetailBean {
    #[serde(rename = "url", default)]
    pub url: String,
    #[serde(rename = "appid", default)]
    pub app_id: String,
    #[serde(rename = "path", default)]
    pub path: String,
    #[serde(rename = "type", default)]
    pub r#type: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OrderItemListBean {
    #[serde(rename = "merchant_item_id", default)]
    pub merchant_item_id: String,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "description", default)]
    pub description: String,
    #[serde(rename = "unit_price", default)]
    pub unit_price: i64,
    #[serde(rename = "quantity", default)]
    pub quantity: i64,
    #[serde(rename = "image_url", default)]
    pub image_url: Vec<String>,
}
