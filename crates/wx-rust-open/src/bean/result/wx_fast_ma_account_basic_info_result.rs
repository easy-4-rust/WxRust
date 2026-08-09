//! 对应 Java `me.chanjar.weixin.open.bean.result.WxFastMaAccountBasicInfoResult.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxFastMaAccountBasicInfoResult {
    #[serde(rename = "errcode", default)]
    pub errcode: String,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "appid", default)]
    pub app_id: String,
    #[serde(rename = "account_type", default)]
    pub account_type: i32,
    #[serde(rename = "principal_type", default)]
    pub principal_type: i32,
    #[serde(rename = "principal_name", default)]
    pub principal_name: String,
    #[serde(rename = "realname_status", default)]
    pub realname_status: i32,
    #[serde(rename = "nickname", default)]
    pub nickname: String,
    #[serde(rename = "wx_verify_info", default)]
    pub wx_verify_info: WxVerifyInfo,
    #[serde(rename = "signature_info", default)]
    pub signature_info: SignatureInfo,
    #[serde(rename = "head_image_info", default)]
    pub head_image_info: HeadImageInfo,
    #[serde(rename = "nickname_info", default)]
    pub nickname_info: NicknameInfo,
    #[serde(rename = "registered_country", default)]
    pub registered_country: i32,
    #[serde(rename = "credential", default)]
    pub credential: String,
    #[serde(rename = "customer_type", default)]
    pub customer_type: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NicknameInfo {
    #[serde(rename = "nickname", default)]
    pub nickname: String,
    #[serde(rename = "modify_used_count", default)]
    pub modify_used_count: i32,
    #[serde(rename = "modify_quota", default)]
    pub modify_quota: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxVerifyInfo {
    #[serde(rename = "qualification_verify", default)]
    pub qualification_verify: bool,
    #[serde(rename = "naming_verify", default)]
    pub naming_verify: bool,
    #[serde(rename = "annual_review", default)]
    pub annual_review: bool,
    #[serde(rename = "annual_review_begin_time", default)]
    pub annual_review_begin_time: String,
    #[serde(rename = "annual_review_end_time", default)]
    pub annual_review_end_time: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SignatureInfo {
    #[serde(rename = "signature", default)]
    pub signature: String,
    #[serde(rename = "modify_used_count", default)]
    pub modify_used_count: i32,
    #[serde(rename = "modify_quota", default)]
    pub modify_quota: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct HeadImageInfo {
    #[serde(rename = "head_image_url", default)]
    pub head_image_url: String,
    #[serde(rename = "modify_used_count", default)]
    pub modify_used_count: i32,
    #[serde(rename = "modify_quota", default)]
    pub modify_quota: i32,
}
