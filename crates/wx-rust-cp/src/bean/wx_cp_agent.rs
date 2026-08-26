//! 对应 Java `me.chanjar.weixin.cp.bean.WxCpAgent.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpAgent {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "agentid", default)]
    pub agent_id: i64,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "square_logo_url", default)]
    pub square_logo_url: String,
    #[serde(rename = "logo_mediaid", default)]
    pub logo_media_id: String,
    #[serde(rename = "description", default)]
    pub description: String,
    #[serde(rename = "allow_userinfos", default)]
    pub allow_user_infos: Users,
    #[serde(rename = "allow_partys", default)]
    pub allow_parties: Parties,
    #[serde(rename = "allow_tags", default)]
    pub allow_tags: Tags,
    #[serde(rename = "close", default)]
    pub close: i32,
    #[serde(rename = "redirect_domain", default)]
    pub redirect_domain: String,
    #[serde(rename = "report_location_flag", default)]
    pub report_location_flag: i32,
    #[serde(rename = "isreportenter", default)]
    pub is_report_enter: i32,
    #[serde(rename = "home_url", default)]
    pub home_url: String,
    #[serde(rename = "customized_publish_status", default)]
    pub customized_publish_status: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Users {
    #[serde(rename = "user", default)]
    pub users: Vec<crate::bean::wx_cp_agent::User>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct User {
    #[serde(rename = "userid", default)]
    pub user_id: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Parties {
    #[serde(rename = "partyid", default)]
    pub party_ids: Vec<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Tags {
    #[serde(rename = "tagid", default)]
    pub tag_ids: Vec<i32>,
}

impl WxCpAgent {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpAgent 解析失败: {e}"))
    }
}

impl WxCpAgent {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpAgent 序列化失败: {e}"))
    }
}
