//! 对应 Java `cn.binarywang.wx.miniapp.bean.invoice.reimburse.UpdateStatusBatchRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
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

impl UpdateStatusBatchRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("UpdateStatusBatchRequest 序列化失败: {e}"))
    }
}
