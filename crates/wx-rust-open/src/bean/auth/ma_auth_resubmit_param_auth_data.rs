//! 对应 Java `me.chanjar.weixin.open.bean.auth.MaAuthResubmitParamAuthData.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MaAuthResubmitParamAuthData {
    #[serde(rename = "customer_type", default)]
    pub customer_type: i32,
    #[serde(rename = "contact_info", default)]
    pub contact_info: MaAuthSubmitParamContactInfo,
    #[serde(rename = "invoice_info", default)]
    pub invoice_info: MaAuthSubmitParamInvoiceInfo,
    #[serde(rename = "qualification", default)]
    pub qualification: String,
    #[serde(rename = "qualification_other", default)]
    pub qualification_other: Vec<String>,
    #[serde(rename = "account_name", default)]
    pub account_name: String,
    #[serde(rename = "account_name_type", default)]
    pub account_name_type: i32,
    #[serde(rename = "account_supplemental", default)]
    pub account_supplemental: Vec<String>,
    #[serde(rename = "pay_type", default)]
    pub pay_type: i32,
    #[serde(rename = "auth_identification", default)]
    pub auth_identification: String,
    #[serde(rename = "auth_ident_material", default)]
    pub auth_ident_material: String,
    #[serde(rename = "third_party_phone", default)]
    pub third_party_phone: String,
    #[serde(rename = "service_appid", default)]
    pub service_app_id: String,
    #[serde(rename = "taskid", default)]
    pub task_id: String,
}
