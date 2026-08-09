//! 对应 Java `cn.binarywang.wx.miniapp.bean.shop.request.shipping.WxMaOrderShippingInfoUploadRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::shop::request::*;
#[allow(unused_imports)]
use crate::bean::shop::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaOrderShippingInfoUploadRequest {
    #[serde(rename = "order_key", default)]
    pub order_key: OrderKeyBean,
    #[serde(rename = "logistics_type", default)]
    pub logistics_type: i32,
    #[serde(rename = "delivery_mode", default)]
    pub delivery_mode: i32,
    #[serde(rename = "is_all_delivered", default)]
    pub is_all_delivered: bool,
    #[serde(rename = "shipping_list", default)]
    pub shipping_list: Vec<ShippingListBean>,
    #[serde(rename = "upload_time", default)]
    pub upload_time: String,
    #[serde(rename = "payer", default)]
    pub payer: PayerBean,
}
