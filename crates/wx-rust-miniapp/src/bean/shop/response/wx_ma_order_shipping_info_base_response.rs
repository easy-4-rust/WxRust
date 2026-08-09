//! 对应 Java `cn.binarywang.wx.miniapp.bean.shop.response.WxMaOrderShippingInfoBaseResponse.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::shop::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaOrderShippingInfoBaseResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Order {
    #[serde(rename = "transaction_id", default)]
    pub transaction_id: String,
    #[serde(rename = "merchant_id", default)]
    pub merchant_id: String,
    #[serde(rename = "sub_merchant_id", default)]
    pub sub_merchant_id: String,
    #[serde(rename = "merchant_trade_no", default)]
    pub merchant_trade_no: String,
    #[serde(rename = "description", default)]
    pub description: String,
    #[serde(rename = "paid_amount", default)]
    pub paid_amount: i64,
    #[serde(rename = "openid", default)]
    pub open_id: String,
    #[serde(rename = "trade_create_time", default)]
    pub trade_create_time: i64,
    #[serde(rename = "pay_time", default)]
    pub pay_time: i64,
    #[serde(rename = "order_state", default)]
    pub order_state: i32,
    #[serde(rename = "in_complaint", default)]
    pub in_complaint: bool,
    #[serde(rename = "shipping", default)]
    pub shipping: Shipping,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Shipping {
    #[serde(rename = "delivery_mode", default)]
    pub delivery_mode: i32,
    #[serde(rename = "logistics_type", default)]
    pub logistics_type: i32,
    #[serde(rename = "finish_shipping", default)]
    pub finish_shipping: bool,
    #[serde(rename = "goods_desc", default)]
    pub goods_desc: String,
    #[serde(rename = "finish_shipping_count", default)]
    pub finish_shipping_count: i32,
    #[serde(rename = "shipping_list", default)]
    pub shipping_list: Vec<ShippingItem>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ShippingItem {
    #[serde(rename = "tracking_no", default)]
    pub tracking_no: String,
    #[serde(rename = "express_company", default)]
    pub express_company: String,
    #[serde(rename = "goods_desc", default)]
    pub goods_desc: String,
    #[serde(rename = "upload_time", default)]
    pub upload_time: i64,
    #[serde(rename = "contact", default)]
    pub contact: Contact,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Contact {
    #[serde(rename = "consignor_contact", default)]
    pub consignor_contact: String,
    #[serde(rename = "receiver_contact", default)]
    pub receiver_contact: String,
}
