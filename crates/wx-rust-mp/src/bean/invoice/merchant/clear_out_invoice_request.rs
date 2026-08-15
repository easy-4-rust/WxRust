//! 对应 Java `bean.invoice.merchant.ClearOutInvoiceRequest`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ClearOutInvoiceRequest {
    #[serde(rename = "invoiceinfo", default)]
    pub invoiceinfo: ClearOutInvoiceInfo,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ClearOutInvoiceInfo {
    #[serde(rename = "wxopenid", default)]
    pub wxopenid: String,
    #[serde(rename = "fpqqlsh", default)]
    pub fpqqlsh: String,
    #[serde(rename = "nsrsbh", default)]
    pub nsrsbh: String,
    #[serde(rename = "nsrmc", default)]
    pub nsrmc: String,
    #[serde(rename = "yfpdm", default)]
    pub yfpdm: String,
    #[serde(rename = "yfphm", default)]
    pub yfphm: String,
}
