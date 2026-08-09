//! 对应 Java `me.chanjar.weixin.open.bean.minishop.MinishopMerchantinfo.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MinishopMerchantinfo {
    #[serde(rename = "merchantId", default)]
    pub merchant_id: i64,
    #[serde(rename = "appId", default)]
    pub app_id: String,
    #[serde(rename = "subjectType", default)]
    pub subject_type: String,
    #[serde(rename = "merchantShortname", default)]
    pub merchant_shortname: String,
    #[serde(rename = "supplementaryDesc", default)]
    pub supplementary_desc: String,
    #[serde(rename = "busiLicenseId", default)]
    pub busi_license_id: i32,
    #[serde(rename = "organizationCodeInfo", default)]
    pub organization_code_info: i32,
    #[serde(rename = "idCardInfo", default)]
    pub id_card_info: i32,
    #[serde(rename = "superAdministratorInfoId", default)]
    pub super_administrator_info_id: i32,
    #[serde(rename = "specialQualificationId", default)]
    pub special_qualification_id: i32,
    #[serde(rename = "supplementaryMaterialId", default)]
    pub supplementary_material_id: i32,
    #[serde(rename = "status", default)]
    pub status: i32,
    #[serde(rename = "submitTime", default)]
    pub submit_time: String,
}
