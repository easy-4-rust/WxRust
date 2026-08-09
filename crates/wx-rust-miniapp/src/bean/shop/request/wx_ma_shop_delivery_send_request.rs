//! 对应 Java `cn.binarywang.wx.miniapp.bean.shop.request.WxMaShopDeliverySendRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::shop::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaShopDeliverySendRequest {
    #[serde(rename = "order_id", default)]
    pub order_id: i64,
    #[serde(rename = "out_order_id", default)]
    pub out_order_id: String,
    #[serde(rename = "openid", default)]
    pub openid: String,
    #[serde(rename = "finish_all_delivery", default)]
    pub finish_all_delivery: i32,
    #[serde(rename = "delivery_list", default)]
    pub delivery_list: Vec<DeliveryListBean>,
    #[serde(rename = "ship_done_time", default)]
    pub ship_done_tme: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DeliveryListBean {
    #[serde(rename = "delivery_id", default)]
    pub delivery_id: String,
    #[serde(rename = "waybill_id", default)]
    pub waybill_id: String,
    #[serde(rename = "product_info_list", default)]
    pub product_info_list: Vec<ProductInfosBean>,
}
