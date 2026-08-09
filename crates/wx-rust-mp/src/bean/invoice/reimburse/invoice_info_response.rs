//! 对应 Java `bean.invoice.reimburse.InvoiceInfoResponse`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
use crate::bean::invoice::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InvoiceInfoResponse {
    #[serde(rename = "card_id", default)]
    pub card_id: String,
    #[serde(rename = "begin_time", default)]
    pub begin_time: i32,
    #[serde(rename = "end_time", default)]
    pub end_time: i32,
    #[serde(rename = "openid", default)]
    pub openid: String,
    #[serde(rename = "type", default)]
    pub r#type: String,
    #[serde(rename = "payee", default)]
    pub payee: String,
    #[serde(rename = "detail", default)]
    pub detail: String,
    #[serde(rename = "user_info", default)]
    pub user_info: InvoiceUserInfo,
}

impl InvoiceInfoResponse {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("InvoiceInfoResponse 解析失败: {e}"))
    }
}
