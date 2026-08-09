//! 对应 Java `me.chanjar.weixin.cp.bean.external.contact.WxCpExternalContactInfo.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::external::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpExternalContactInfo {
    #[serde(rename = "external_contact", default)]
    pub external_contact: crate::bean::wx_cp_user_external_contact_info::ExternalContact,
    #[serde(rename = "follow_user", default)]
    pub followed_users: Vec<crate::bean::wx_cp_user_external_contact_info::FollowedUser>,
    #[serde(rename = "next_cursor", default)]
    pub next_cursor: String,
}

impl WxCpExternalContactInfo {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpExternalContactInfo 解析失败: {e}"))
    }
}
