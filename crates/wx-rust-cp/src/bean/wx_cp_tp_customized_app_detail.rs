//! 对应 Java `me.chanjar.weixin.cp.bean.WxCpTpCustomizedAppDetail.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpTpCustomizedAppDetail {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "auth_corpid", default)]
    pub auth_corp_id: String,
    #[serde(rename = "auth_corp_name", default)]
    pub auth_corp_name: String,
    #[serde(rename = "auth_corp_square_logo_url", default)]
    pub auth_corp_square_logo_url: String,
    #[serde(rename = "auth_corp_round_logo_url", default)]
    pub auth_corp_round_logo_url: String,
    #[serde(rename = "auth_corp_type", default)]
    pub auth_corp_type: i32,
    #[serde(rename = "auth_corp_qrcode_url", default)]
    pub auth_corp_qrcode_url: String,
    #[serde(rename = "auth_corp_user_limit", default)]
    pub auth_corp_user_limit: i32,
    #[serde(rename = "auth_corp_full_name", default)]
    pub auth_corp_full_name: String,
    #[serde(rename = "auth_corp_verified_type", default)]
    pub auth_corp_verified_type: i32,
    #[serde(rename = "auth_corp_industry", default)]
    pub auth_corp_industry: String,
    #[serde(rename = "auth_corp_sub_industry", default)]
    pub auth_corp_sub_industry: String,
    #[serde(rename = "auth_corp_location", default)]
    pub auth_corp_location: String,
    #[serde(rename = "customized_app_list", default)]
    pub customized_app_list: Vec<CustomizedApp>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CustomizedApp {
    #[serde(rename = "agentid", default)]
    pub agent_id: i32,
    #[serde(rename = "template_id", default)]
    pub template_id: String,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "description", default)]
    pub description: String,
    #[serde(rename = "logo_url", default)]
    pub logo_url: String,
    #[serde(rename = "allow_userinfos", default)]
    pub allow_user_infos: crate::bean::wx_cp_tp_customized_app_detail::AllowUserInfos,
    #[serde(rename = "close", default)]
    pub close: i32,
    #[serde(rename = "home_url", default)]
    pub home_url: String,
    #[serde(rename = "app_type", default)]
    pub app_type: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AllowUserInfos {
    #[serde(rename = "user", default)]
    pub users: Vec<crate::bean::wx_cp_tp_customized_app_detail::User>,
    #[serde(rename = "department", default)]
    pub departments: Vec<crate::bean::wx_cp_tp_customized_app_detail::Department>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct User {
    #[serde(rename = "userid", default)]
    pub user_id: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Department {
    #[serde(rename = "id", default)]
    pub id: i32,
}

impl WxCpTpCustomizedAppDetail {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpTpCustomizedAppDetail 解析失败: {e}"))
    }
}

impl WxCpTpCustomizedAppDetail {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxCpTpCustomizedAppDetail 序列化失败: {e}"))
    }
}
