//! 对应 Java `cn.binarywang.wx.miniapp.bean.shop.request.WxMaShopAuditBrandRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::shop::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaShopAuditBrandRequest {
    #[serde(rename = "audit_req", default)]
    pub audit_req: AuditReqBean,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AuditReqBean {
    #[serde(rename = "brand_info", default)]
    pub brand_info: BrandInfoBean,
    #[serde(rename = "license", default)]
    pub license: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BrandInfoBean {
    #[serde(rename = "brand_audit_type", default)]
    pub brand_audit_type: i32,
    #[serde(rename = "trademark_type", default)]
    pub trademark_type: String,
    #[serde(rename = "brand_management_type", default)]
    pub brand_management_type: i32,
    #[serde(rename = "commodity_origin_type", default)]
    pub commodity_origin_type: i32,
    #[serde(rename = "brand_wording", default)]
    pub brand_wording: String,
    #[serde(rename = "trademark_registrant", default)]
    pub trademark_registrant: String,
    #[serde(rename = "trademark_registrant_nu", default)]
    pub trademark_registrant_nu: String,
    #[serde(rename = "trademark_authorization_period", default)]
    pub trademark_authorization_period: String,
    #[serde(rename = "trademark_applicant", default)]
    pub trademark_applicant: String,
    #[serde(rename = "trademark_application_time", default)]
    pub trademark_application_time: String,
    #[serde(rename = "sale_authorization", default)]
    pub sale_authorization: Vec<String>,
    #[serde(rename = "trademark_registration_certificate", default)]
    pub trademark_registration_certificate: Vec<String>,
    #[serde(rename = "trademark_change_certificate", default)]
    pub trademark_change_certificate: Vec<String>,
    #[serde(rename = "trademark_registration_application", default)]
    pub trademark_registration_application: Vec<String>,
    #[serde(rename = "imported_goods_form", default)]
    pub imported_goods_form: Vec<String>,
}
