//! 对应 Java `bean.invoice.reimburse.InvoiceUserInfo`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
use crate::bean::invoice::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InvoiceUserInfo {
    #[serde(rename = "fee", default)]
    pub fee: i32,
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "billing_time", default)]
    pub billing_time: i32,
    #[serde(rename = "billing_no", default)]
    pub billing_no: String,
    #[serde(rename = "billing_code", default)]
    pub billing_code: String,
    #[serde(rename = "fee_without_tax", default)]
    pub fee_without_tax: i32,
    #[serde(rename = "tax", default)]
    pub tax: i32,
    #[serde(rename = "pdf_url", default)]
    pub pdf_url: String,
    #[serde(rename = "trip_pdf_url", default)]
    pub trip_pdf_url: String,
    #[serde(rename = "reimburse_status", default)]
    pub reimburse_status: String,
    #[serde(rename = "check_code", default)]
    pub check_code: String,
    #[serde(rename = "buyer_number", default)]
    pub buyer_number: String,
    #[serde(rename = "buyer_address_and_phone", default)]
    pub buyer_address_and_phone: String,
    #[serde(rename = "buyer_bank_account", default)]
    pub buyer_bank_account: String,
    #[serde(rename = "seller_number", default)]
    pub seller_number: String,
    #[serde(rename = "seller_address_and_phone", default)]
    pub seller_address_and_phone: String,
    #[serde(rename = "seller_bank_account", default)]
    pub seller_bank_account: String,
    #[serde(rename = "remarks", default)]
    pub remarks: String,
    #[serde(rename = "cashier", default)]
    pub cashier: String,
    #[serde(rename = "maker", default)]
    pub maker: String,
    #[serde(rename = "info", default)]
    pub info: Vec<InvoiceCommodityInfo>,
}
