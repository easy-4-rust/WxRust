//! 对应 Java `me.chanjar.weixin.cp.bean.WxTpLoginInfo.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxTpLoginInfo {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "usertype", default)]
    pub user_type: i32,
    #[serde(rename = "user_info", default)]
    pub user_info: UserInfo,
    #[serde(rename = "corp_info", default)]
    pub corp_info: CorpInfoBean,
    #[serde(rename = "auth_info", default)]
    pub auth_info: AuthInfo,
    #[serde(rename = "agent", default)]
    pub agent: Vec<Agent>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UserInfo {
    #[serde(rename = "userid", default)]
    pub user_id: String,
    #[serde(rename = "open_userid", default)]
    pub open_user_id: String,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "avatar", default)]
    pub avatar: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CorpInfoBean {
    #[serde(rename = "corpid", default)]
    pub corp_id: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AuthInfo {
    #[serde(rename = "department", default)]
    pub department: Vec<Department>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Department {
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "writable", default)]
    pub writable: bool,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Agent {
    #[serde(rename = "agentid", default)]
    pub agent_id: i32,
    #[serde(rename = "auth_type", default)]
    pub auth_type: i32,
}

impl WxTpLoginInfo {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxTpLoginInfo 解析失败: {e}"))
    }
}
