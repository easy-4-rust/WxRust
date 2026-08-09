//! 对应 Java `me.chanjar.weixin.open.bean.ma.privacy.PrivacyOwnerSetting.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::ma::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PrivacyOwnerSetting {
    #[serde(rename = "contact_email", default)]
    pub contact_email: String,
    #[serde(rename = "contact_phone", default)]
    pub contact_phone: String,
    #[serde(rename = "contact_qq", default)]
    pub contact_qq: String,
    #[serde(rename = "contact_weixin", default)]
    pub contact_weixin: String,
    #[serde(rename = "ext_file_media_id", default)]
    pub ext_file_media_id: String,
    #[serde(rename = "notice_method", default)]
    pub notice_method: String,
    #[serde(rename = "store_expire_timestamp", default)]
    pub store_expire_timestamp: String,
}
