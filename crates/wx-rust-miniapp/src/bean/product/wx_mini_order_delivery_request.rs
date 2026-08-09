//! 对应 Java `cn.binarywang.wx.miniapp.bean.product.WxMiniOrderDeliveryRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMiniOrderDeliveryRequest {
    #[serde(rename = "order_id", default)]
    pub order_id: i64,
    #[serde(rename = "delivery_list", default)]
    pub delivery_list: Vec<DeliveryListBean>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DeliveryListBean {
    #[serde(rename = "delivery_id", default)]
    pub delivery_id: String,
    #[serde(rename = "is_all_product", default)]
    pub is_all_product: bool,
    #[serde(rename = "waybill_id", default)]
    pub waybill_id: String,
    #[serde(rename = "product_infos", default)]
    pub product_info_list: Vec<ProductInfosBean>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProductInfosBean {
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    #[serde(rename = "sku_id", default)]
    pub sku_id: String,
    #[serde(rename = "product_cnt", default)]
    pub product_cnt: i32,
}
