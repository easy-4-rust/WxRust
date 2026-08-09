//! 对应 Java `me.chanjar.weixin.open.bean.auth.MaAuthSubmitParamInvoiceInfo.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MaAuthSubmitParamInvoiceInfo {
    #[serde(rename = "invoice_type", default)]
    pub invoice_type: i32,
    #[serde(rename = "electronic", default)]
    pub electronic: MaAuthSubmitParamInvoiceElectronic,
    #[serde(rename = "vat", default)]
    pub vat: MaAuthSubmitParamInvoiceVat,
    #[serde(rename = "invoice_title", default)]
    pub invoice_title: String,
}
