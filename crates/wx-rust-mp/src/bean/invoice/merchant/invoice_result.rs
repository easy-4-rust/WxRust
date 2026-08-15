//! 对应 Java `bean.invoice.merchant.InvoiceResult`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InvoiceResult {
    #[serde(rename = "invoicedetail", default)]
    pub invoicedetail: InvoiceDetail,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InvoiceDetail {
    #[serde(rename = "fpqqlsh", default)]
    pub fpqqlsh: String,
    #[serde(rename = "jym", default)]
    pub jym: String,
    #[serde(rename = "kprq", default)]
    pub kprq: String,
    #[serde(rename = "fpdm", default)]
    pub fpdm: String,
    #[serde(rename = "fphm", default)]
    pub fphm: String,
    #[serde(rename = "pdfurl", default)]
    pub pdfurl: String,
}

impl InvoiceResult {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("InvoiceResult 解析失败: {e}"))
    }
}
