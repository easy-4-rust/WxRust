//! 对应 Java `bean.invoice.reimburse.InvoiceInfoRequest`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InvoiceInfoRequest {
    #[serde(rename = "card_id", default)]
    pub card_id: String,
    #[serde(rename = "encrypt_code", default)]
    pub encrypt_code: String,
}
