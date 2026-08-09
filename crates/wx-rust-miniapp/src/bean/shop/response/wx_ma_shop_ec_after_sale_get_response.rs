//! 对应 Java `cn.binarywang.wx.miniapp.bean.shop.response.WxMaShopEcAfterSaleGetResponse.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::shop::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaShopEcAfterSaleGetResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "after_sales_order", default)]
    pub after_sales_order: AfterSalesOrderDTO,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AfterSalesOrderDTO {
    #[serde(rename = "out_aftersale_id", default)]
    pub out_aftersale_id: String,
    #[serde(rename = "aftersale_id", default)]
    pub aftersale_id: i64,
    #[serde(rename = "out_order_id", default)]
    pub out_order_id: String,
    #[serde(rename = "order_id", default)]
    pub order_id: i64,
    #[serde(rename = "product_info", default)]
    pub product_info: ProductInfoDTO,
    #[serde(rename = "type", default)]
    pub r#type: i32,
    #[serde(rename = "return_info", default)]
    pub return_info: ReturnInfoDTO,
    #[serde(rename = "orderamt", default)]
    pub orderamt: i32,
    #[serde(rename = "refund_reason_type", default)]
    pub refund_reason_type: i32,
    #[serde(rename = "refund_reason", default)]
    pub refund_reason: String,
    #[serde(rename = "status", default)]
    pub status: i32,
    #[serde(rename = "create_time", default)]
    pub create_time: String,
    #[serde(rename = "update_time", default)]
    pub update_time: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProductInfoDTO {
    #[serde(rename = "out_product_id", default)]
    pub out_product_id: String,
    #[serde(rename = "out_sku_id", default)]
    pub out_sku_id: String,
    #[serde(rename = "product_cnt", default)]
    pub product_cnt: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ReturnInfoDTO {
    #[serde(rename = "order_return_time", default)]
    pub order_return_time: String,
    #[serde(rename = "delivery_id", default)]
    pub delivery_id: String,
    #[serde(rename = "waybill_id", default)]
    pub waybill_id: String,
    #[serde(rename = "delivery_name", default)]
    pub delivery_name: String,
}
