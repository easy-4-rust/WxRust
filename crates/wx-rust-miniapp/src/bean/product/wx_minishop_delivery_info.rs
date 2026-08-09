//! 对应 Java `cn.binarywang.wx.miniapp.bean.product.WxMinishopDeliveryInfo.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMinishopDeliveryInfo {
    #[serde(rename = "address_info", default)]
    pub address_info: WxMinishopAddressInfo,
    #[serde(rename = "delivery_method", default)]
    pub delivery_method: String,
    #[serde(rename = "delivery_product_info", default)]
    pub delivery_product_info: Vec<DeliveryProductInfo>,
    #[serde(rename = "ship_done_time", default)]
    pub ship_done_time: i64,
    #[serde(rename = "insurance_info", default)]
    pub insurance_info: InsuranceInfo,
    #[serde(rename = "deliver_type", default)]
    pub deliver_type: String,
    #[serde(rename = "offline_delivery_time", default)]
    pub offline_delivery_time: i64,
    #[serde(rename = "offline_pickup_time", default)]
    pub offline_pickup_time: i64,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DeliveryProductInfo {
    #[serde(rename = "waybill_id", default)]
    pub waybill_id: String,
    #[serde(rename = "delivery_id", default)]
    pub delivery_id: String,
    #[serde(rename = "delivery_time", default)]
    pub delivery_time: String,
    #[serde(rename = "deliver_type", default)]
    pub deliver_type: String,
    #[serde(rename = "delivery_address", default)]
    pub delivery_address: WxMinishopAddressInfo,
    #[serde(rename = "product_infos", default)]
    pub product_infos: Vec<ProductInfo>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InsuranceInfo {
    #[serde(rename = "type", default)]
    pub r#type: String,
    #[serde(rename = "insurance_price", default)]
    pub insurance_price: i64,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProductInfo {
    #[serde(rename = "product_id", default)]
    pub product_id: i64,
    #[serde(rename = "sku_id", default)]
    pub sku_id: i64,
    #[serde(rename = "product_cnt", default)]
    pub product_cnt: i64,
}
