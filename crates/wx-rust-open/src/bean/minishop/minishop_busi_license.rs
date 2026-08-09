//! 对应 Java `me.chanjar.weixin.open.bean.minishop.MinishopBusiLicense.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MinishopBusiLicense {
    #[serde(rename = "busiLicenseId", default)]
    pub busi_license_id: i32,
    #[serde(rename = "licenseType", default)]
    pub license_type: i32,
    #[serde(rename = "picFile", default)]
    pub pic_file: MinishopPicFile,
    #[serde(rename = "picFileUrl", default)]
    pub pic_file_url: String,
    #[serde(rename = "registrationNum", default)]
    pub registration_num: String,
    #[serde(rename = "merchantName", default)]
    pub merchant_name: String,
    #[serde(rename = "legalRepresentative", default)]
    pub legal_representative: String,
    #[serde(rename = "registeredAddrs", default)]
    pub registered_addrs: String,
    #[serde(rename = "startDate", default)]
    pub start_date: String,
    #[serde(rename = "endDate", default)]
    pub end_date: String,
}
