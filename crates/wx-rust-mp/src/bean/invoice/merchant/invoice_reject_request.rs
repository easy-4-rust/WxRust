//! 对应 Java `bean.invoice.merchant.InvoiceRejectRequest`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
use crate::bean::invoice::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InvoiceRejectRequest {
    #[serde(rename = "s_pappid", default)]
    pub s_pappid: String,
    #[serde(rename = "order_id", default)]
    pub order_id: String,
    #[serde(rename = "reason", default)]
    pub reason: String,
    #[serde(rename = "url", default)]
    pub url: String,
}
