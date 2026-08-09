//! 对应 Java `bean.invoice.reimburse.UpdateStatusBatchRequest`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
use crate::bean::invoice::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateStatusBatchRequest {
    #[serde(rename = "openid", default)]
    pub openid: String,
    #[serde(rename = "reimburse_status", default)]
    pub reimburse_status: String,
    #[serde(rename = "invoice_list", default)]
    pub invoice_list: Vec<InvoiceInfoRequest>,
}
