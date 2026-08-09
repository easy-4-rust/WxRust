//! 对应 Java `me.chanjar.weixin.open.bean.ma.privacy.GetPrivacySettingResult.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::ma::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetPrivacySettingResult {
    #[serde(rename = "errcode", default)]
    pub errcode: String,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "code_exist", default)]
    pub code_exist: i32,
    #[serde(rename = "privacy_list", default)]
    pub privacy_list: Vec<String>,
    #[serde(rename = "setting_list", default)]
    pub setting_list: Vec<Setting>,
    #[serde(rename = "update_time", default)]
    pub update_time: i64,
    #[serde(rename = "owner_setting", default)]
    pub owner_setting: PrivacyOwnerSetting,
    #[serde(rename = "privacy_desc", default)]
    pub privacy_desc: PrivacyDesc,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Setting {
    #[serde(rename = "privacy_key", default)]
    pub privacy_key: String,
    #[serde(rename = "privacy_text", default)]
    pub privacy_text: String,
    #[serde(rename = "privacy_label", default)]
    pub privacy_label: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PrivacyDesc {
    #[serde(rename = "privacy_desc_list", default)]
    pub privacy_desc_list: Vec<PrivacyDescItem>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PrivacyDescItem {
    #[serde(rename = "privacy_key", default)]
    pub privacy_key: String,
    #[serde(rename = "privacy_desc", default)]
    pub privacy_desc: String,
}
