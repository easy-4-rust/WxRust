//! 对应 Java `me.chanjar.weixin.cp.bean.school.user.WxCpAllowScope.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::school::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpAllowScope {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "allow_scope", default)]
    pub allow_scope: AllowScope,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AllowScope {
    #[serde(rename = "students", default)]
    pub students: Vec<crate::bean::school::user::wx_cp_allow_scope::Student>,
    #[serde(rename = "departments", default)]
    pub departments: crate::bean::school::user::wx_cp_allow_scope::Department,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Department {
    #[serde(rename = "partyid", default)]
    pub party_id: Vec<i32>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Student {
    #[serde(rename = "userid", default)]
    pub user_id: String,
}

impl WxCpAllowScope {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpAllowScope 解析失败: {e}"))
    }
}

impl WxCpAllowScope {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpAllowScope 序列化失败: {e}"))
    }
}
