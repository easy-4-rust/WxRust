//! 对应 Java `bean.invoice.merchant.InvoiceAuthPageRequest`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InvoiceAuthPageRequest {
    #[serde(rename = "s_pappid", default)]
    pub s_pappid: String,
    #[serde(rename = "order_id", default)]
    pub order_id: String,
    #[serde(rename = "money", default)]
    pub money: i64,
    #[serde(rename = "source", default)]
    pub source: String,
    #[serde(rename = "redirect_url", default)]
    pub redirect_url: String,
    #[serde(rename = "type", default)]
    pub r#type: i32,
    #[serde(rename = "timestamp", default)]
    pub timestamp: i64,
    #[serde(rename = "ticket", default)]
    pub ticket: String,
}
