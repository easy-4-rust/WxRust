//! 对应 Java `cn.binarywang.wx.miniapp.bean.delivery.AddOrderRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddOrderRequest {
    #[serde(rename = "delivery_id", default)]
    pub delivery_id: String,
    #[serde(rename = "shop_order_id", default)]
    pub shop_order_id: String,
    #[serde(rename = "openid", default)]
    pub openid: String,
    #[serde(rename = "shop_no", default)]
    pub shop_no: String,
    #[serde(rename = "shopid", default)]
    pub shop_id: String,
    #[serde(rename = "delivery_sign", default)]
    pub delivery_sign: String,
    #[serde(rename = "appSecret", default)]
    pub app_secret: String,
    #[serde(rename = "sub_biz_id", default)]
    pub sub_biz_id: String,
    #[serde(rename = "sender", default)]
    pub sender: Sender,
    #[serde(rename = "receiver", default)]
    pub receiver: Receiver,
    #[serde(rename = "cargo", default)]
    pub cargo: Cargo,
    #[serde(rename = "order_info", default)]
    pub order_info: OrderInfo,
    #[serde(rename = "shop", default)]
    pub shop: Shop,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Sender {
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "city", default)]
    pub city: String,
    #[serde(rename = "address", default)]
    pub address: String,
    #[serde(rename = "address_detail", default)]
    pub address_detail: String,
    #[serde(rename = "coordinate_type", default)]
    pub coordinate_type: i32,
    #[serde(
        rename = "lng",
        default,
        serialize_with = "crate::bean::serde_util::ser_decimal"
    )]
    pub lng: String,
    #[serde(
        rename = "lat",
        default,
        serialize_with = "crate::bean::serde_util::ser_decimal"
    )]
    pub lat: String,
    #[serde(rename = "phone", default)]
    pub phone: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Receiver {
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "city", default)]
    pub city: String,
    #[serde(rename = "address", default)]
    pub address: String,
    #[serde(rename = "address_detail", default)]
    pub address_detail: String,
    #[serde(rename = "coordinate_type", default)]
    pub coordinate_type: i32,
    #[serde(
        rename = "lng",
        default,
        serialize_with = "crate::bean::serde_util::ser_decimal"
    )]
    pub lng: String,
    #[serde(
        rename = "lat",
        default,
        serialize_with = "crate::bean::serde_util::ser_decimal"
    )]
    pub lat: String,
    #[serde(rename = "phone", default)]
    pub phone: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Shop {
    #[serde(rename = "goods_count", default)]
    pub goods_count: i32,
    #[serde(rename = "goods_name", default)]
    pub goods_name: String,
    #[serde(rename = "img_url", default)]
    pub img_url: String,
    #[serde(rename = "wxa_path", default)]
    pub wxa_path: String,
    #[serde(rename = "wxa_appid", default)]
    pub wxa_appid: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OrderInfo {
    #[serde(rename = "delivery_service_code", default)]
    pub delivery_service_code: String,
    #[serde(rename = "order_type", default)]
    pub order_type: i32,
    #[serde(rename = "expected_delivery_time", default)]
    pub expected_delivery_time: i64,
    #[serde(rename = "expected_finish_time", default)]
    pub expected_finish_time: i64,
    #[serde(rename = "expected_pick_time", default)]
    pub expected_pick_time: i64,
    #[serde(rename = "note", default)]
    pub note: String,
    #[serde(rename = "poi_seq", default)]
    pub poi_seq: String,
    #[serde(rename = "order_time", default)]
    pub order_time: i64,
    #[serde(rename = "is_insured", default)]
    pub is_insured: i32,
    #[serde(
        rename = "declared_value",
        default,
        serialize_with = "crate::bean::serde_util::ser_decimal"
    )]
    pub declared_value: String,
    #[serde(rename = "tips", default)]
    pub tips: i32,
    #[serde(rename = "is_direct_delivery", default)]
    pub is_direct_delivery: i32,
    #[serde(
        rename = "cash_on_delivery",
        default,
        serialize_with = "crate::bean::serde_util::ser_decimal"
    )]
    pub cash_on_delivery: String,
    #[serde(
        rename = "cash_on_pickup",
        default,
        serialize_with = "crate::bean::serde_util::ser_decimal"
    )]
    pub cash_on_pickup: String,
    #[serde(rename = "rider_pick_method", default)]
    pub rider_pick_method: i32,
    #[serde(rename = "is_finish_code_needed", default)]
    pub is_finish_code_needed: i32,
    #[serde(rename = "is_pickup_code_needed", default)]
    pub is_pickup_code_needed: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Cargo {
    #[serde(
        rename = "goods_value",
        default,
        serialize_with = "crate::bean::serde_util::ser_decimal"
    )]
    pub goods_value: String,
    #[serde(
        rename = "goods_height",
        default,
        serialize_with = "crate::bean::serde_util::ser_decimal"
    )]
    pub goods_height: String,
    #[serde(
        rename = "goods_length",
        default,
        serialize_with = "crate::bean::serde_util::ser_decimal"
    )]
    pub goods_length: String,
    #[serde(
        rename = "goods_width",
        default,
        serialize_with = "crate::bean::serde_util::ser_decimal"
    )]
    pub goods_width: String,
    #[serde(
        rename = "goods_weight",
        default,
        serialize_with = "crate::bean::serde_util::ser_decimal"
    )]
    pub goods_weight: String,
    #[serde(rename = "goods_detail", default)]
    pub goods_detail: GoodsDetail,
    #[serde(rename = "goods_pickup_info", default)]
    pub goods_pickup_info: String,
    #[serde(rename = "goods_delivery_info", default)]
    pub goods_delivery_info: String,
    #[serde(rename = "cargo_first_class", default)]
    pub cargo_first_class: String,
    #[serde(rename = "cargo_second_class", default)]
    pub cargo_second_class: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GoodsDetail {
    #[serde(rename = "goods", default)]
    pub goods: Vec<Goods>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Goods {
    #[serde(rename = "good_count", default)]
    pub good_count: i32,
    #[serde(rename = "good_name", default)]
    pub good_name: String,
    #[serde(
        rename = "good_price",
        default,
        serialize_with = "crate::bean::serde_util::ser_decimal"
    )]
    pub good_price: String,
    #[serde(rename = "good_unit", default)]
    pub good_unit: String,
}
