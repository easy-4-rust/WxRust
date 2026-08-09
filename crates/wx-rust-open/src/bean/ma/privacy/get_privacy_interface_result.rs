//! 对应 Java `me.chanjar.weixin.open.bean.ma.privacy.GetPrivacyInterfaceResult.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::ma::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetPrivacyInterfaceResult {
    #[serde(rename = "errcode", default)]
    pub errcode: String,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "interface_list", default)]
    pub interface_list: Vec<Interface>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Interface {
    #[serde(rename = "api_name", default)]
    pub api_name: String,
    #[serde(rename = "api_ch_name", default)]
    pub api_ch_name: String,
    #[serde(rename = "api_desc", default)]
    pub api_desc: String,
    #[serde(rename = "apply_time", default)]
    pub apply_time: String,
    #[serde(rename = "status", default)]
    pub status: String,
    #[serde(rename = "audit_id", default)]
    pub audit_id: String,
    #[serde(rename = "fail_reason", default)]
    pub fail_reason: String,
    #[serde(rename = "api_link", default)]
    pub api_link: String,
    #[serde(rename = "group_name", default)]
    pub group_name: String,
}
