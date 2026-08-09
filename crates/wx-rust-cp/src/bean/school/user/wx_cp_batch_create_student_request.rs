//! 对应 Java `me.chanjar.weixin.cp.bean.school.user.WxCpBatchCreateStudentRequest.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::school::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpBatchCreateStudentRequest {
    #[serde(rename = "students", default)]
    pub students: Vec<Student>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Student {
    #[serde(rename = "student_userid", default)]
    pub student_user_id: String,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "department", default)]
    pub department: Vec<i32>,
}

impl WxCpBatchCreateStudentRequest {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json)
            .map_err(|e| format!("WxCpBatchCreateStudentRequest 解析失败: {e}"))
    }
}

impl WxCpBatchCreateStudentRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxCpBatchCreateStudentRequest 序列化失败: {e}"))
    }
}
