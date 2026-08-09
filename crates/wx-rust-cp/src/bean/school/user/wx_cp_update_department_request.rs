//! 对应 Java `me.chanjar.weixin.cp.bean.school.user.WxCpUpdateDepartmentRequest.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::school::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpUpdateDepartmentRequest {
    #[serde(rename = "parentid", default)]
    pub parent_id: i32,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "new_id", default)]
    pub new_id: i32,
    #[serde(rename = "type", default)]
    pub r#type: i32,
    #[serde(rename = "register_year", default)]
    pub register_year: i32,
    #[serde(rename = "standard_grade", default)]
    pub standard_grade: i32,
    #[serde(rename = "order", default)]
    pub order: i32,
    #[serde(rename = "department_admins", default)]
    pub department_admins: Vec<DepartmentAdmin>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DepartmentAdmin {
    #[serde(rename = "userid", default)]
    pub user_id: String,
    #[serde(rename = "op", default)]
    pub op: i32,
    #[serde(rename = "type", default)]
    pub r#type: i32,
    #[serde(rename = "subject", default)]
    pub subject: String,
}

impl WxCpUpdateDepartmentRequest {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpUpdateDepartmentRequest 解析失败: {e}"))
    }
}

impl WxCpUpdateDepartmentRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxCpUpdateDepartmentRequest 序列化失败: {e}"))
    }
}
