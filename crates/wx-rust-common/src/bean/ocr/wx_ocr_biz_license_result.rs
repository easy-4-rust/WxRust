//! 对应 Java `me.chanjar.weixin.common.bean.ocr.WxOcrBizLicenseResult`（由 gen_bean_structs.py 生成）。

use super::wx_ocr_img_size::WxOcrImgSize;
use super::wx_ocr_pos::WxOcrPos;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxOcrBizLicenseResult {
    /// regNum
    #[serde(rename = "reg_num", default)]
    pub reg_num: String,
    /// serial
    #[serde(rename = "serial", default)]
    pub serial: String,
    /// legalRepresentative
    #[serde(rename = "legal_representative", default)]
    pub legal_representative: String,
    /// enterpriseName
    #[serde(rename = "enterprise_name", default)]
    pub enterprise_name: String,
    /// typeOfOrganization
    #[serde(rename = "type_of_organization", default)]
    pub type_of_organization: String,
    /// address
    #[serde(rename = "address", default)]
    pub address: String,
    /// typeOfEnterprise
    #[serde(rename = "type_of_enterprise", default)]
    pub type_of_enterprise: String,
    /// businessScope
    #[serde(rename = "business_scope", default)]
    pub business_scope: String,
    /// registeredCapital
    #[serde(rename = "registered_capital", default)]
    pub registered_capital: String,
    /// paidInCapital
    #[serde(rename = "paid_in_capital", default)]
    pub paid_in_capital: String,
    /// validPeriod
    #[serde(rename = "valid_period", default)]
    pub valid_period: String,
    /// registeredDate
    #[serde(rename = "registered_date", default)]
    pub registered_date: String,
    /// certPosition
    #[serde(rename = "cert_position", default)]
    pub cert_position: CertPosition,
    /// imgSize
    #[serde(rename = "img_size", default)]
    pub img_size: WxOcrImgSize,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CertPosition {
    /// pos
    #[serde(rename = "pos", default)]
    pub pos: WxOcrPos,
}
