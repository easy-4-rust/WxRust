//! 对应 Java `bean.invoice.merchant.InvoiceAuthDataResult`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InvoiceAuthDataResult {
    #[serde(rename = "invoiceStatus", default)]
    pub invoice_status: String,
    #[serde(rename = "authTime", default)]
    pub auth_time: i64,
    #[serde(rename = "userAuthInfo", default)]
    pub user_auth_info: UserAuthInfo,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UserAuthInfo {
    #[serde(rename = "userField", default)]
    pub user_field: UserField,
    #[serde(rename = "bizField", default)]
    pub biz_field: BizField,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UserField {
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "phone", default)]
    pub phone: String,
    #[serde(rename = "email", default)]
    pub email: String,
    #[serde(rename = "customField", default)]
    pub custom_field: Vec<KeyValuePair>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BizField {
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "taxNo", default)]
    pub tax_no: String,
    #[serde(rename = "addr", default)]
    pub addr: String,
    #[serde(rename = "phone", default)]
    pub phone: String,
    #[serde(rename = "bankType", default)]
    pub bank_type: String,
    #[serde(rename = "bankNo", default)]
    pub bank_no: String,
    #[serde(rename = "customField", default)]
    pub custom_field: Vec<KeyValuePair>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct KeyValuePair {
    #[serde(rename = "key", default)]
    pub key: String,
    #[serde(rename = "value", default)]
    pub value: String,
}

impl InvoiceAuthDataResult {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("InvoiceAuthDataResult 解析失败: {e}"))
    }
}
