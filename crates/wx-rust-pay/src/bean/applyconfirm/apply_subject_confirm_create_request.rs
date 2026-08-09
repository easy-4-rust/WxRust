//! 对应 Java `com.github.binarywang.wxpay.bean.applyconfirm.ApplySubjectConfirmCreateRequest.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApplySubjectConfirmCreateRequest {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "channel_id"
    )]
    pub channel_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "business_code"
    )]
    pub business_code: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "contact_info"
    )]
    pub contact_info: Option<ApplySubConfirmContactInfo>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "subject_info"
    )]
    pub subject_info: Option<ApplySubConfirmSubjectInfo>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "identification_info"
    )]
    pub identity_info: Option<ApplySubConfirmIdentificationInfo>,
    #[serde(default, rename = "ubo_info_list")]
    pub ubo_info_list: Vec<ApplySubConfirmUboInfo>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "addition_info"
    )]
    pub addition_info: Option<ApplySubConfirmAdditionInfo>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApplySubConfirmIdentificationInfo {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "id_holder_type"
    )]
    pub id_holder_type: Option<String>,
    #[serde(default, rename = "identification_type")]
    pub identification_type: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "authorize_letter_copy"
    )]
    pub authorize_letter_copy: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "identification_name"
    )]
    pub identification_name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "identification_number"
    )]
    pub identification_number: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "identification_valid_date"
    )]
    pub identification_valid_date: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "identification_address"
    )]
    pub identification_address: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "identification_front_copy"
    )]
    pub identification_front_copy: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "identification_back_copy"
    )]
    pub identification_back_copy: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "owner")]
    pub owner: Option<bool>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApplySubConfirmUboInfo {
    #[serde(default, rename = "ubo_id_doc_type")]
    pub ubo_id_doc_type: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "ubo_id_doc_copy"
    )]
    pub ubo_id_doc_copy: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "ubo_id_doc_copy_back"
    )]
    pub ubo_id_doc_copy_back: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "ubo_id_doc_name"
    )]
    pub ubo_id_doc_name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "ubo_id_doc_number"
    )]
    pub ubo_id_doc_number: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "ubo_id_doc_address"
    )]
    pub ubo_id_doc_address: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "ubo_period_begin"
    )]
    pub ubo_period_begin: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "ubo_period_end"
    )]
    pub ubo_period_end: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApplySubConfirmContactInfo {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "contact_type"
    )]
    pub contact_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "name")]
    pub name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "contact_id_doc_type"
    )]
    pub contact_id_doc_type: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "id_card_number"
    )]
    pub contact_id_number: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "contact_id_doc_copy"
    )]
    pub contact_id_doc_copy: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "contact_id_doc_copy_back"
    )]
    pub contact_id_doc_copy_back: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "contact_period_begin"
    )]
    pub contact_period_begin: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "contact_period_end"
    )]
    pub contact_period_end: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mobile")]
    pub mobile: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApplySubConfirmSubjectInfo {
    #[serde(default, rename = "subject_type")]
    pub subject_type: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "is_finance_institution"
    )]
    pub finance_institution: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "business_licence_info"
    )]
    pub business_license_info: Option<ApplySubConfirmBusinessLicenseInfo>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "certificate_info"
    )]
    pub certificate_info: Option<ApplySubConfirmCertificateInfo>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "company_prove_copy"
    )]
    pub company_prove_copy: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "assist_prove_info"
    )]
    pub assist_prove_info: Option<ApplySubConfirmAssistProveInfo>,
    #[serde(default, rename = "special_operation_list")]
    pub special_operation_list: Vec<ApplySubConfirmSpecialOperationList>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "finance_institution_info"
    )]
    pub finance_institution_info: Option<ApplySubConfirmFinanceInstitutionInfo>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApplySubConfirmBusinessLicenseInfo {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "license_number"
    )]
    pub license_number: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "license_copy"
    )]
    pub license_copy: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "merchant_name"
    )]
    pub merchant_name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "legal_person"
    )]
    pub legal_person: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "company_address"
    )]
    pub company_address: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "licence_valid_date"
    )]
    pub period_begin: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApplySubConfirmCertificateInfo {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cert_copy")]
    pub cert_copy: Option<String>,
    #[serde(default, rename = "cert_type")]
    pub cert_type: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "cert_number"
    )]
    pub cert_number: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "merchant_name"
    )]
    pub merchant_name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "company_address"
    )]
    pub company_address: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "legal_person"
    )]
    pub legal_person: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "cert_valid_date"
    )]
    pub cert_valid_date: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApplySubConfirmAssistProveInfo {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "micro_biz_type"
    )]
    pub micro_biz_type: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "store_name"
    )]
    pub store_name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "store_address_code"
    )]
    pub store_address_code: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "store_address"
    )]
    pub store_address: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "store_header_copy"
    )]
    pub store_header_copy: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "store_indoor_copy"
    )]
    pub store_indoor_copy: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApplySubConfirmSpecialOperationList {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "category_id"
    )]
    pub category_id: Option<i32>,
    #[serde(default, rename = "operation_copy_list")]
    pub finance_license_pics: Vec<Option<String>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApplySubConfirmFinanceInstitutionInfo {
    #[serde(default, rename = "finance_type")]
    pub finance_type: String,
    #[serde(default, rename = "finance_license_pics")]
    pub finance_license_pics: Vec<Option<String>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApplySubConfirmAdditionInfo {
    #[serde(default, rename = "confirm_mchid_list")]
    pub confirm_mchid_list: Vec<Option<String>>,
}
