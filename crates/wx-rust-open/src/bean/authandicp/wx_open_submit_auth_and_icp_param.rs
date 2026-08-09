//! 对应 Java `me.chanjar.weixin.open.bean.authandicp.WxOpenSubmitAuthAndIcpParam.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxOpenSubmitAuthAndIcpParam {
    #[serde(rename = "auth_data", default)]
    pub auth_data: AuthData,
    #[serde(rename = "icp_subject", default)]
    pub icp_subject: IcpSubject,
    #[serde(rename = "icp_applets", default)]
    pub icp_applets: IcpApplets,
    #[serde(rename = "icp_materials", default)]
    pub icp_materials: IcpMaterials,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AuthData {
    #[serde(rename = "contact_info", default)]
    pub contact_info: ContactInfo,
    #[serde(rename = "invoice_info", default)]
    pub invoice_info: InvoiceInfo,
    #[serde(rename = "customer_type", default)]
    pub customer_type: i32,
    #[serde(rename = "pay_type", default)]
    pub pay_type: i32,
    #[serde(rename = "qualification_other", default)]
    pub qualification_other: Vec<String>,
    #[serde(rename = "account_name", default)]
    pub account_name: String,
    #[serde(rename = "account_name_type", default)]
    pub account_name_type: String,
    #[serde(rename = "account_supplemental", default)]
    pub account_supplemental: Vec<String>,
    #[serde(rename = "auth_identification", default)]
    pub auth_identification: String,
    #[serde(rename = "auth_ident_material", default)]
    pub auth_ident_material: Vec<String>,
    #[serde(rename = "third_party_phone", default)]
    pub third_party_phone: String,
    #[serde(rename = "service_appid", default)]
    pub service_appid: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ContactInfo {
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "email", default)]
    pub email: String,
    #[serde(rename = "mobile", default)]
    pub mobile: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InvoiceInfo {
    #[serde(rename = "invoice_type", default)]
    pub invoice_type: String,
    #[serde(rename = "electronic", default)]
    pub electronic: Electronic,
    #[serde(rename = "vat", default)]
    pub vat: Vat,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Electronic {
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "desc", default)]
    pub desc: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Vat {
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
    #[serde(rename = "desc", default)]
    pub desc: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct IcpSubject {
    #[serde(rename = "base_info", default)]
    pub base_info: SubjectBaseInfo,
    #[serde(rename = "personal_info", default)]
    pub personal_info: SubjectPersonalInfo,
    #[serde(rename = "organize_info", default)]
    pub organize_info: SubjectOrganizeInfo,
    #[serde(rename = "principal_info", default)]
    pub principal_info: SubjectPrincipalInfo,
    #[serde(rename = "legal_person_info", default)]
    pub legal_person_info: SubjectLegalPersonInfo,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SubjectBaseInfo {
    #[serde(rename = "type", default)]
    pub r#type: i32,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "province", default)]
    pub province: String,
    #[serde(rename = "city", default)]
    pub city: String,
    #[serde(rename = "district", default)]
    pub district: String,
    #[serde(rename = "address", default)]
    pub address: String,
    #[serde(rename = "comment", default)]
    pub comment: String,
    #[serde(rename = "record_number", default)]
    pub record_number: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SubjectPersonalInfo {
    #[serde(rename = "residence_permit", default)]
    pub residence_permit: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SubjectOrganizeInfo {
    #[serde(rename = "certificate_type", default)]
    pub certificate_type: i32,
    #[serde(rename = "certificate_number", default)]
    pub certificate_number: String,
    #[serde(rename = "certificate_address", default)]
    pub certificate_address: String,
    #[serde(rename = "certificate_photo", default)]
    pub certificate_photo: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SubjectPrincipalInfo {
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "mobile", default)]
    pub mobile: String,
    #[serde(rename = "email", default)]
    pub email: String,
    #[serde(rename = "emergency_contact", default)]
    pub emergency_contact: String,
    #[serde(rename = "certificate_type", default)]
    pub certificate_type: i32,
    #[serde(rename = "certificate_number", default)]
    pub certificate_number: String,
    #[serde(rename = "certificate_validity_date_start", default)]
    pub certificate_validity_date_start: String,
    #[serde(rename = "certificate_validity_date_end", default)]
    pub certificate_validity_date_end: String,
    #[serde(rename = "certificate_photo_front", default)]
    pub certificate_photo_front: String,
    #[serde(rename = "certificate_photo_back", default)]
    pub certificate_photo_back: String,
    #[serde(rename = "authorization_letter", default)]
    pub authorization_letter: String,
    #[serde(rename = "verify_task_id", default)]
    pub verify_task_id: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SubjectLegalPersonInfo {
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "certificate_number", default)]
    pub certificate_number: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct IcpApplets {
    #[serde(rename = "base_info", default)]
    pub base_info: AppletsBaseInfo,
    #[serde(rename = "principal_info", default)]
    pub principal_info: AppletsPrincipalInfo,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AppletsBaseInfo {
    #[serde(rename = "appid", default)]
    pub app_id: String,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "service_content_types", default)]
    pub service_content_types: Vec<i32>,
    #[serde(rename = "nrlx_details", default)]
    pub nrlx_details: Vec<AppletsNrlxDetailItem>,
    #[serde(rename = "comment", default)]
    pub comment: String,
    #[serde(rename = "record_number", default)]
    pub record_number: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AppletsNrlxDetailItem {
    #[serde(rename = "type", default)]
    pub r#type: i32,
    #[serde(rename = "code", default)]
    pub code: String,
    #[serde(rename = "media", default)]
    pub media: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AppletsPrincipalInfo {
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "mobile", default)]
    pub mobile: String,
    #[serde(rename = "email", default)]
    pub email: String,
    #[serde(rename = "emergency_contact", default)]
    pub emergency_contact: String,
    #[serde(rename = "certificate_type", default)]
    pub certificate_type: i32,
    #[serde(rename = "certificate_number", default)]
    pub certificate_number: String,
    #[serde(rename = "certificate_validity_date_start", default)]
    pub certificate_validity_date_start: String,
    #[serde(rename = "certificate_validity_date_end", default)]
    pub certificate_validity_date_end: String,
    #[serde(rename = "certificate_photo_front", default)]
    pub certificate_photo_front: String,
    #[serde(rename = "certificate_photo_back", default)]
    pub certificate_photo_back: String,
    #[serde(rename = "authorization_letter", default)]
    pub authorization_letter: String,
    #[serde(rename = "verify_task_id", default)]
    pub verify_task_id: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct IcpMaterials {
    #[serde(rename = "commitment_letter", default)]
    pub commitment_letter: Vec<String>,
    #[serde(rename = "business_name_change_letter", default)]
    pub business_name_change_letter: Vec<String>,
    #[serde(rename = "party_building_confirmation_letter", default)]
    pub party_building_confirmation_letter: Vec<String>,
    #[serde(rename = "promise_video", default)]
    pub promise_video: Vec<String>,
    #[serde(rename = "authenticity_responsibility_letter", default)]
    pub authenticity_responsibility_letter: Vec<String>,
    #[serde(rename = "authenticity_commitment_letter", default)]
    pub authenticity_commitment_letter: Vec<String>,
    #[serde(rename = "website_construction_proposal", default)]
    pub website_construction_proposal: Vec<String>,
    #[serde(rename = "subject_other_materials", default)]
    pub subject_other_materials: Vec<String>,
    #[serde(rename = "applets_other_materials", default)]
    pub applets_other_materials: Vec<String>,
    #[serde(rename = "holding_certificate_photo", default)]
    pub holding_certificate_photo: Vec<String>,
}
