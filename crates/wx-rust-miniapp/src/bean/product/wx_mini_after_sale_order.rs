//! 对应 Java `cn.binarywang.wx.miniapp.bean.product.WxMiniAfterSaleOrder.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMiniAfterSaleOrder {
    #[serde(rename = "order_id", default)]
    pub order_id: i64,
    #[serde(rename = "status", default)]
    pub status: String,
    #[serde(rename = "openid", default)]
    pub openid: String,
    #[serde(rename = "original_order_id", default)]
    pub original_order_id: i64,
    #[serde(rename = "product_info", default)]
    pub product_info: AfterSaleProductInfo,
    #[serde(rename = "details", default)]
    pub details: AfterSaleDetails,
    #[serde(rename = "refund_info", default)]
    pub refund_info: RefundInfo,
    #[serde(rename = "return_info", default)]
    pub return_info: ReturnInfo,
    #[serde(rename = "merchant_upload_info", default)]
    pub merchant_upload_info: MerchantUploadInfo,
    #[serde(rename = "create_time", default)]
    pub create_time: i64,
    #[serde(rename = "update_time", default)]
    pub update_time: i64,
    #[serde(rename = "reason", default)]
    pub reason: String,
    #[serde(rename = "refund_resp", default)]
    pub refund_resp: RefundResp,
    #[serde(rename = "type", default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AfterSaleProductInfo {
    #[serde(rename = "product_id", default)]
    pub product_id: i64,
    #[serde(rename = "sku_id", default)]
    pub sku_id: i64,
    #[serde(rename = "count", default)]
    pub count: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AfterSaleDetails {
    #[serde(rename = "num", default)]
    pub num: i32,
    #[serde(rename = "desc", default)]
    pub desc: String,
    #[serde(rename = "cancel_time", default)]
    pub cancel_time: i64,
    #[serde(rename = "prove_imgs", default)]
    pub prove_imgs: Vec<String>,
    #[serde(rename = "tel_number", default)]
    pub tel_number: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RefundInfo {
    #[serde(rename = "amount", default)]
    pub amount: i64,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ReturnInfo {
    #[serde(rename = "delivery_id", default)]
    pub delivery_id: String,
    #[serde(rename = "waybill_id", default)]
    pub waybill_id: String,
    #[serde(rename = "delivery_name", default)]
    pub delivery_name: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MerchantUploadInfo {
    #[serde(rename = "reject_reason", default)]
    pub reject_reason: String,
    #[serde(rename = "refund_certificates", default)]
    pub refund_certificates: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RefundResp {
    #[serde(rename = "code", default)]
    pub code: String,
    #[serde(rename = "ret", default)]
    pub ret: i32,
    #[serde(rename = "message", default)]
    pub message: String,
}
