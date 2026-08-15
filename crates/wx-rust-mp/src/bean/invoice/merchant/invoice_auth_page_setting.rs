//! 对应 Java `bean.invoice.merchant.InvoiceAuthPageSetting`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InvoiceAuthPageSetting {
    #[serde(rename = "authField", default)]
    pub auth_field: AuthField,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AuthField {
    #[serde(rename = "userField", default)]
    pub user_field: UserField,
    #[serde(rename = "bizField", default)]
    pub biz_field: BizField,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UserField {
    #[serde(rename = "showTitle", default)]
    pub show_title: i32,
    #[serde(rename = "showPhone", default)]
    pub show_phone: i32,
    #[serde(rename = "showEmail", default)]
    pub show_email: i32,
    #[serde(rename = "requirePhone", default)]
    pub require_phone: i32,
    #[serde(rename = "requireEmail", default)]
    pub require_email: i32,
    #[serde(rename = "customField", default)]
    pub custom_field: Vec<KeyValuePair>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BizField {
    #[serde(rename = "showTitle", default)]
    pub show_title: i32,
    #[serde(rename = "showTaxNo", default)]
    pub show_tax_no: i32,
    #[serde(rename = "showAddr", default)]
    pub show_addr: i32,
    #[serde(rename = "showPhone", default)]
    pub show_phone: i32,
    #[serde(rename = "showBankType", default)]
    pub show_bank_type: i32,
    #[serde(rename = "showBankNo", default)]
    pub show_bank_no: i32,
    #[serde(rename = "requireTaxNo", default)]
    pub require_tax_no: i32,
    #[serde(rename = "requireAddr", default)]
    pub require_addr: i32,
    #[serde(rename = "requirePhone", default)]
    pub require_phone: i32,
    #[serde(rename = "requireBankType", default)]
    pub require_bank_type: i32,
    #[serde(rename = "requireBankNo", default)]
    pub require_bank_no: i32,
    #[serde(rename = "customField", default)]
    pub custom_field: Vec<KeyValuePair>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CustomField {
    #[serde(rename = "key", default)]
    pub key: String,
    #[serde(rename = "isRequire", default)]
    pub is_require: i32,
    #[serde(rename = "notice", default)]
    pub notice: String,
}
