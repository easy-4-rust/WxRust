//! 对应 Java `com.github.binarywang.wxpay.bean.applyment.WxPayApplyment4SubCreateRequest.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxPayApplyment4SubCreateRequest {
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
    pub contact_info: Option<ContactInfo>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "subject_info"
    )]
    pub subject_info: Option<SubjectInfo>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "business_info"
    )]
    pub business_info: Option<BusinessInfo>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "settlement_info"
    )]
    pub settlement_info: Option<SettlementInfo>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "bank_account_info"
    )]
    pub bank_account_info: Option<BankAccountInfo>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "addition_info"
    )]
    pub addition_info: Option<AdditionInfo>,
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
        rename = "contact_id_number"
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
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "business_authorization_letter"
    )]
    pub business_authorization_letter: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "openid")]
    pub openid: Option<String>,
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
pub struct SubjectInfo {
    #[serde(default, rename = "subject_type")]
    pub subject_type: String,
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
        rename = "certificate_info"
    )]
    pub certificate_info: Option<CertificateInfo>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "certificate_letter_copy"
    )]
    pub certificate_letter_copy: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "micro_biz_info"
    )]
    pub micro_biz_info: Option<MicroBizInfo>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "finance_institution_info"
    )]
    pub finance_institution_info: Option<FinanceInstitutionInfo>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "identity_info"
    )]
    pub identity_info: Option<IdentityInfo>,
    #[serde(default, rename = "ubo_info_list")]
    pub ubo_info_list: Vec<UboInfo>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BusinessLicenseInfo {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "license_copy"
    )]
    pub license_copy: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "license_number"
    )]
    pub license_number: Option<String>,
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
        rename = "license_address"
    )]
    pub license_address: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "period_begin"
    )]
    pub period_begin: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "period_end"
    )]
    pub period_end: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CertificateInfo {
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
        rename = "period_begin"
    )]
    pub period_begin: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "period_end"
    )]
    pub period_end: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FinanceInstitutionInfo {
    #[serde(default, rename = "finance_type")]
    pub finance_type: String,
    #[serde(default, rename = "finance_license_pics")]
    pub finance_license_pics: Vec<Option<String>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MicroBizInfo {
    #[serde(default, rename = "micro_biz_type")]
    pub micro_biz_type: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "micro_store_info"
    )]
    pub micro_store_info: Option<MicroStoreInfo>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "micro_mobile_info"
    )]
    pub micro_mobile_info: Option<MicroMobileInfo>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "micro_online_info"
    )]
    pub micro_online_info: Option<MicroOnlineInfo>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MicroOnlineInfo {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "micro_online_store"
    )]
    pub micro_online_store: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "micro_ec_name"
    )]
    pub micro_ec_name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "micro_qrcode"
    )]
    pub micro_qrcode: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "micro_link"
    )]
    pub micro_link: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MicroMobileInfo {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "micro_mobile_name"
    )]
    pub micro_mobile_name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "micro_mobile_city"
    )]
    pub micro_mobile_city: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "micro_mobile_address"
    )]
    pub micro_mobile_address: Option<String>,
    #[serde(default, rename = "micro_mobile_pics")]
    pub micro_mobile_pics: Vec<Option<String>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MicroStoreInfo {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "micro_name"
    )]
    pub micro_name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "micro_address_code"
    )]
    pub micro_address_code: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "micro_address"
    )]
    pub micro_address: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "store_entrance_pic"
    )]
    pub store_entrance_pic: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "micro_indoor_copy"
    )]
    pub micro_indoor_copy: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "store_longitude"
    )]
    pub store_longitude: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "store_latitude"
    )]
    pub store_latitude: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct IdentityInfo {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "id_holder_type"
    )]
    pub id_holder_type: Option<String>,
    #[serde(default, rename = "id_doc_type")]
    pub id_doc_type: String,
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
        rename = "card_period_begin"
    )]
    pub card_period_begin: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "card_period_end"
    )]
    pub card_period_end: Option<String>,
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
pub struct BusinessInfo {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "merchant_shortname"
    )]
    pub merchant_shortname: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "service_phone"
    )]
    pub service_phone: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "sales_info"
    )]
    pub sales_info: Option<SalesInfo>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SalesInfo {
    #[serde(default, rename = "sales_scenes_type")]
    pub sales_scenes_type: Vec<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "biz_store_info"
    )]
    pub biz_store_info: Option<BizStoreInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mp_info")]
    pub mp_info: Option<MpInfo>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "mini_program_info"
    )]
    pub mini_program_info: Option<MiniProgramInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "app_info")]
    pub app_info: Option<AppInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "web_info")]
    pub web_info: Option<WebInfo>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "wework_info"
    )]
    pub wework_info: Option<WeworkInfo>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BizStoreInfo {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "biz_store_name"
    )]
    pub biz_store_name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "biz_address_code"
    )]
    pub biz_address_code: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "biz_store_address"
    )]
    pub biz_store_address: Option<String>,
    #[serde(default, rename = "store_entrance_pic")]
    pub store_entrance_pic: Vec<Option<String>>,
    #[serde(default, rename = "indoor_pic")]
    pub indoor_pic: Vec<Option<String>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "biz_sub_appid"
    )]
    pub biz_sub_appid: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MpInfo {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mp_appid")]
    pub mp_appid: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "mp_sub_appid"
    )]
    pub mp_sub_appid: Option<String>,
    #[serde(default, rename = "mp_pics")]
    pub mp_pics: Vec<Option<String>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MiniProgramInfo {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "mini_program_appid"
    )]
    pub mini_program_appid: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "mini_program_sub_appid"
    )]
    pub mini_program_sub_appid: Option<String>,
    #[serde(default, rename = "mini_program_pics")]
    pub mini_program_pics: Vec<Option<String>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AppInfo {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "app_appid")]
    pub app_appid: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "app_sub_appid"
    )]
    pub app_sub_appid: Option<String>,
    #[serde(default, rename = "app_pics")]
    pub app_pics: Vec<Option<String>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WebInfo {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "domain")]
    pub domain: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "web_authorisation"
    )]
    pub web_authorisation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "web_appid")]
    pub web_appid: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WeworkInfo {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "sub_corp_id"
    )]
    pub sub_corp_id: Option<String>,
    #[serde(default, rename = "wework_pics")]
    pub wework_pics: Vec<Option<String>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SettlementInfo {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "settlement_id"
    )]
    pub settlement_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "qualification_type"
    )]
    pub qualification_type: Option<String>,
    #[serde(default, rename = "qualifications")]
    pub qualifications: Vec<Option<String>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "activities_id"
    )]
    pub activities_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "activities_rate"
    )]
    pub activities_rate: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "debit_activities_rate"
    )]
    pub debit_activities_rate: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "credit_activities_rate"
    )]
    pub credit_activities_rate: Option<String>,
    #[serde(default, rename = "activities_additions")]
    pub activities_additions: Vec<Option<String>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BankAccountInfo {
    #[serde(default, rename = "bank_account_type")]
    pub bank_account_type: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "account_name"
    )]
    pub account_name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "account_bank"
    )]
    pub account_bank: Option<String>,
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
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AdditionInfo {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "legal_person_commitment"
    )]
    pub legal_person_commitment: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "legal_person_video"
    )]
    pub legal_person_video: Option<String>,
    #[serde(default, rename = "business_addition_pics")]
    pub business_addition_pics: Vec<Option<String>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "business_addition_msg"
    )]
    pub business_addition_msg: Option<String>,
}
