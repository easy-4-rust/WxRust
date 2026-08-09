//! 对应 Java `me.chanjar.weixin.open.bean.auth.MaAuthSubmitParamInvoiceVat.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MaAuthSubmitParamInvoiceVat {
    #[serde(rename = "enterprise_phone", default)]
    pub enterprise_phone: String,
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "enterprise_address", default)]
    pub enterprise_address: String,
    #[serde(rename = "bank_name", default)]
    pub bank_name: String,
    #[serde(rename = "bank_account", default)]
    pub bank_account: String,
    #[serde(rename = "mailing_address", default)]
    pub mailing_address: String,
    #[serde(rename = "address", default)]
    pub address: String,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "phone", default)]
    pub phone: String,
    #[serde(rename = "province", default)]
    pub province: String,
    #[serde(rename = "city", default)]
    pub city: String,
    #[serde(rename = "district", default)]
    pub district: String,
    #[serde(rename = "desc", default)]
    pub desc: String,
}
