//! 对应 Java `com.github.binarywang.wxpay.bean.ecommerce.ApplymentsRequest.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApplymentsRequest {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "out_request_no"
    )]
    pub out_request_no: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "organization_type"
    )]
    pub organization_type: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "finance_institution"
    )]
    pub finance_institution: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "business_license_info"
    )]
    pub business_license_info: Option<BusinessLicenseInfo>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "finance_institution_info"
    )]
    pub finance_institution_info: Option<FinanceInstitutionInfo>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "id_holder_type"
    )]
    pub id_holder_type: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "id_doc_type"
    )]
    pub id_doc_type: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "authorize_letter_copy"
    )]
    pub authorize_letter_copy: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "id_card_info"
    )]
    pub id_card_info: Option<IdCardInfo>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "id_doc_info"
    )]
    pub id_doc_info: Option<IdDocInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "owner")]
    pub owner: Option<bool>,
    #[serde(default, rename = "ubo_info_list")]
    pub ubo_info_list: Vec<UboInfo>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "account_info"
    )]
    pub account_info: Option<AccountInfo>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "contact_info"
    )]
    pub contact_info: Option<ContactInfo>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "sales_scene_info"
    )]
    pub sales_scene_info: Option<SalesSceneInfo>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "settlement_info"
    )]
    pub settlement_info: Option<SettlementInfo>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "merchant_shortname"
    )]
    pub merchant_shortname: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "qualifications"
    )]
    pub qualifications: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "business_addition_pics"
    )]
    pub business_addition_pics: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "business_addition_desc"
    )]
    pub business_addition_desc: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BusinessLicenseInfo {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cert_type")]
    pub cert_type: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "business_license_copy"
    )]
    pub business_license_copy: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "business_license_number"
    )]
    pub business_license_number: Option<String>,
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
        rename = "business_time"
    )]
    pub business_time: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FinanceInstitutionInfo {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "finance_type"
    )]
    pub finance_type: Option<String>,
    #[serde(default, rename = "finance_license_pics")]
    pub finance_license_pics: Vec<Option<String>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct IdCardInfo {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "id_card_copy"
    )]
    pub id_card_copy: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "id_card_national"
    )]
    pub id_card_national: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "id_card_name"
    )]
    pub id_card_name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "id_card_number"
    )]
    pub id_card_number: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "id_card_address"
    )]
    pub id_card_address: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "id_card_valid_time_begin"
    )]
    pub id_card_valid_time_begin: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "id_card_valid_time"
    )]
    pub id_card_valid_time: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct IdDocInfo {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "id_doc_copy"
    )]
    pub id_doc_copy: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "id_doc_copy_back"
    )]
    pub id_doc_copy_back: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "id_doc_name"
    )]
    pub id_doc_name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "id_doc_number"
    )]
    pub id_doc_number: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "id_doc_address"
    )]
    pub id_doc_address: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "doc_period_begin"
    )]
    pub doc_period_begin: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "doc_period_end"
    )]
    pub doc_period_end: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UboInfo {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "ubo_id_doc_type"
    )]
    pub ubo_id_doc_type: Option<String>,
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
        rename = "ubo_id_doc_period_begin"
    )]
    pub ubo_id_doc_period_begin: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "ubo_id_doc_period_end"
    )]
    pub ubo_id_doc_period_end: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AccountInfo {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "bank_account_type"
    )]
    pub bank_account_type: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "account_bank"
    )]
    pub account_bank: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "account_name"
    )]
    pub account_name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "bank_address_code"
    )]
    pub bank_address_code: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "bank_branch_id"
    )]
    pub bank_branch_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "bank_name")]
    pub bank_name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "account_number"
    )]
    pub account_number: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "account_cert_info"
    )]
    pub account_cert_info: Option<AccountCertInfo>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AccountCertInfo {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "settlement_cert_pic"
    )]
    pub settlement_cert_pic: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "relation_cert_pic"
    )]
    pub relation_cert_pic: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "other_cert_pics"
    )]
    pub other_cert_pics: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ContactInfo {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "contact_type"
    )]
    pub contact_type: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "contact_name"
    )]
    pub contact_name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "contact_id_doc_type"
    )]
    pub contact_id_doc_type: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "contact_id_card_number"
    )]
    pub contact_id_card_number: Option<String>,
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
        rename = "contact_id_doc_period_begin"
    )]
    pub contact_id_doc_period_begin: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "contact_id_doc_period_end"
    )]
    pub contact_id_doc_period_end: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "business_authorization_letter"
    )]
    pub business_authorization_letter: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "mobile_phone"
    )]
    pub mobile_phone: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "contact_email"
    )]
    pub contact_email: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SalesSceneInfo {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "store_name"
    )]
    pub store_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "store_url")]
    pub store_url: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "store_qr_code"
    )]
    pub store_qr_code: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "mini_program_sub_appid"
    )]
    pub mini_program_sub_appid: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SettlementInfo {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "settlement_id"
    )]
    pub settlement_id: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "qualification_type"
    )]
    pub qualification_type: Option<String>,
}
