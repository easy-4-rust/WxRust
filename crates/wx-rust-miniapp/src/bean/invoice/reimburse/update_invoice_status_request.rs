//! 对应 Java `cn.binarywang.wx.miniapp.bean.invoice.reimburse.UpdateInvoiceStatusRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::invoice::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateInvoiceStatusRequest {
    #[serde(rename = "card_id", default)]
    pub card_id: String,
    #[serde(rename = "encrypt_code", default)]
    pub encrypt_code: String,
    #[serde(rename = "reimburse_status", default)]
    pub reimburse_status: String,
}

impl UpdateInvoiceStatusRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("UpdateInvoiceStatusRequest 序列化失败: {e}"))
    }
}
