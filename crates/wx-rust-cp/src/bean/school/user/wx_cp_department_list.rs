//! 对应 Java `me.chanjar.weixin.cp.bean.school.user.WxCpDepartmentList.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::school::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpDepartmentList {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "departments", default)]
    pub departments: Vec<Department>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Department {
    #[serde(rename = "parentid", default)]
    pub parent_id: i32,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "type", default)]
    pub r#type: i32,
    #[serde(rename = "register_year", default)]
    pub register_year: i32,
    #[serde(rename = "standard_grade", default)]
    pub standard_grade: i32,
    #[serde(rename = "order", default)]
    pub order: i32,
    #[serde(rename = "is_graduated", default)]
    pub is_graduated: i32,
    #[serde(rename = "open_group_chat", default)]
    pub open_group_chat: i32,
    #[serde(rename = "group_chat_id", default)]
    pub group_chat_id: String,
    #[serde(rename = "department_admins", default)]
    pub department_admins: Vec<crate::bean::school::user::wx_cp_department_list::DepartmentAdmin>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DepartmentAdmin {
    #[serde(rename = "userid", default)]
    pub user_id: String,
    #[serde(rename = "type", default)]
    pub r#type: i32,
    #[serde(rename = "subject", default)]
    pub subject: String,
}

impl WxCpDepartmentList {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpDepartmentList 解析失败: {e}"))
    }
}

impl WxCpDepartmentList {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpDepartmentList 序列化失败: {e}"))
    }
}
