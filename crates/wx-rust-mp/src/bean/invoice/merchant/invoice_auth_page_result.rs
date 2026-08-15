//! 对应 Java `bean.invoice.merchant.InvoiceAuthPageResult`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InvoiceAuthPageResult {
    #[serde(rename = "authUrl", default)]
    pub auth_url: String,
    #[serde(rename = "appid", default)]
    pub appid: String,
}

impl InvoiceAuthPageResult {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("InvoiceAuthPageResult 解析失败: {e}"))
    }
}
