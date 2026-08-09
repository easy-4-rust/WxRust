//! 对应 Java `me.chanjar.weixin.open.bean.ma.privacy.SetPrivacySetting.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::ma::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetPrivacySetting {
    #[serde(rename = "privacy_ver", default)]
    pub privacy_ver: i32,
    #[serde(rename = "owner_setting", default)]
    pub owner_setting: PrivacyOwnerSetting,
    #[serde(rename = "setting_list", default)]
    pub setting_list: Vec<Setting>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Setting {
    #[serde(rename = "privacy_key", default)]
    pub privacy_key: String,
    #[serde(rename = "privacy_text", default)]
    pub privacy_text: String,
}
