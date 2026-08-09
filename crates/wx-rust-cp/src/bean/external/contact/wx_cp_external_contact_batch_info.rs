//! 对应 Java `me.chanjar.weixin.cp.bean.external.contact.WxCpExternalContactBatchInfo.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::external::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpExternalContactBatchInfo {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "external_contact_list", default)]
    pub external_contact_list: Vec<ExternalContactInfo>,
    #[serde(rename = "next_cursor", default)]
    pub next_cursor: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExternalContactInfo {
    #[serde(rename = "external_contact", default)]
    pub external_contact: crate::bean::wx_cp_user_external_contact_info::ExternalContact,
    #[serde(rename = "follow_info", default)]
    pub follow_info: crate::bean::wx_cp_user_external_contact_info::FollowedUser,
}

impl WxCpExternalContactBatchInfo {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json)
            .map_err(|e| format!("WxCpExternalContactBatchInfo 解析失败: {e}"))
    }
}
