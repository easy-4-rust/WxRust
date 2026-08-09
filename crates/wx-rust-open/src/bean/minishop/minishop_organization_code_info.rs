//! 对应 Java `me.chanjar.weixin.open.bean.minishop.MinishopOrganizationCodeInfo.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MinishopOrganizationCodeInfo {
    #[serde(rename = "organizationCodeInfoId", default)]
    pub organization_code_info_id: i32,
    #[serde(rename = "appId", default)]
    pub app_id: String,
    #[serde(rename = "picFile", default)]
    pub pic_file: MinishopPicFile,
    #[serde(rename = "organizationCode", default)]
    pub organization_code: String,
    #[serde(rename = "startDate", default)]
    pub start_date: String,
    #[serde(rename = "endDate", default)]
    pub end_date: String,
}
