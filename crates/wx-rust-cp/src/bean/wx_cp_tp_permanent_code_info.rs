//! 对应 Java `me.chanjar.weixin.cp.bean.WxCpTpPermanentCodeInfo.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpTpPermanentCodeInfo {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "access_token", default)]
    pub access_token: String,
    #[serde(rename = "expires_in", default)]
    pub expires_in: i64,
    #[serde(rename = "permanent_code", default)]
    pub permanent_code: String,
    #[serde(rename = "auth_corp_info", default)]
    pub auth_corp_info: AuthCorpInfo,
    #[serde(rename = "auth_info", default)]
    pub auth_info: AuthInfo,
    #[serde(rename = "auth_user_info", default)]
    pub auth_user_info: AuthUserInfo,
    #[serde(rename = "register_code_info", default)]
    pub register_code_info: RegisterCodeInfo,
    #[serde(rename = "edition_info", default)]
    pub edition_info: EditionInfo,
    #[serde(rename = "state", default)]
    pub state: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AuthCorpInfo {
    #[serde(rename = "corpid", default)]
    pub corp_id: String,
    #[serde(rename = "corp_name", default)]
    pub corp_name: String,
    #[serde(rename = "corp_type", default)]
    pub corp_type: String,
    #[serde(rename = "corp_square_logo_url", default)]
    pub corp_square_logo_url: String,
    #[serde(rename = "corp_round_logo_url", default)]
    pub corp_round_logo_url: String,
    #[serde(rename = "corp_user_max", default)]
    pub corp_user_max: String,
    #[serde(rename = "corp_agent_max", default)]
    pub corp_agent_max: String,
    #[serde(rename = "corp_full_name", default)]
    pub corp_full_name: String,
    #[serde(rename = "verified_end_time", default)]
    pub verified_end_time: i64,
    #[serde(rename = "subject_type", default)]
    pub subject_type: i32,
    #[serde(rename = "corp_wxqrcode", default)]
    pub corp_wx_qrcode: String,
    #[serde(rename = "corp_scale", default)]
    pub corp_scale: String,
    #[serde(rename = "corp_industry", default)]
    pub corp_industry: String,
    #[serde(rename = "corp_sub_industry", default)]
    pub corp_sub_industry: String,
    #[serde(rename = "location", default)]
    pub location: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AuthInfo {
    #[serde(rename = "agent", default)]
    pub agents: Vec<crate::bean::wx_cp_tp_permanent_code_info::Agent>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EditionInfo {
    #[serde(rename = "agent", default)]
    pub agents: Vec<crate::bean::wx_cp_tp_permanent_code_info::Agent>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Agent {
    #[serde(rename = "agentid", default)]
    pub agent_id: i32,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "round_logo_url", default)]
    pub round_logo_url: String,
    #[serde(rename = "square_logo_url", default)]
    pub square_logo_url: String,
    #[serde(rename = "appid", default)]
    pub appid: String,
    #[serde(rename = "auth_mode", default)]
    pub auth_mode: i32,
    #[serde(rename = "is_customized_app", default)]
    pub is_customized_app: bool,
    #[serde(rename = "privilege", default)]
    pub privilege: crate::bean::wx_cp_tp_permanent_code_info::Privilege,
    #[serde(rename = "edition_id", default)]
    pub edition_id: String,
    #[serde(rename = "edition_name", default)]
    pub edition_name: String,
    #[serde(rename = "app_status", default)]
    pub app_status: i32,
    #[serde(rename = "user_limit", default)]
    pub user_limit: i64,
    #[serde(rename = "expired_time", default)]
    pub expired_time: i64,
    #[serde(rename = "is_virtual_version", default)]
    pub is_virtual_version: bool,
    #[serde(rename = "is_shared_from_other_corp", default)]
    pub is_shared_from_other_corp: bool,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AuthUserInfo {
    #[serde(rename = "userid", default)]
    pub user_id: String,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "avatar", default)]
    pub avatar: String,
    #[serde(rename = "open_userid", default)]
    pub open_userid: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RegisterCodeInfo {
    #[serde(rename = "register_code", default)]
    pub register_code: String,
    #[serde(rename = "template_id", default)]
    pub template_id: String,
    #[serde(rename = "state", default)]
    pub state: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Privilege {
    #[serde(rename = "level", default)]
    pub level: i32,
    #[serde(rename = "allow_party", default)]
    pub allow_parties: Vec<i32>,
    #[serde(rename = "allow_user", default)]
    pub allow_users: Vec<String>,
    #[serde(rename = "allow_tag", default)]
    pub allow_tags: Vec<i32>,
    #[serde(rename = "extra_party", default)]
    pub extra_parties: Vec<i32>,
    #[serde(rename = "extra_user", default)]
    pub extra_users: Vec<String>,
    #[serde(rename = "extra_tag", default)]
    pub extra_tags: Vec<i32>,
}

impl WxCpTpPermanentCodeInfo {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpTpPermanentCodeInfo 解析失败: {e}"))
    }
}

impl WxCpTpPermanentCodeInfo {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpTpPermanentCodeInfo 序列化失败: {e}"))
    }
}
