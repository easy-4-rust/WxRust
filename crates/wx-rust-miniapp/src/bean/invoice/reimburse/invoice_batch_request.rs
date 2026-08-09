//! 对应 Java `cn.binarywang.wx.miniapp.bean.invoice.reimburse.InvoiceBatchRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::invoice::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InvoiceBatchRequest {
    #[serde(rename = "item_list", default)]
    pub item_list: Vec<InvoiceInfoRequest>,
}

impl InvoiceBatchRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("InvoiceBatchRequest 序列化失败: {e}"))
    }
}
