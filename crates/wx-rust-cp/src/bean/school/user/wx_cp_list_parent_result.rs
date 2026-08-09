//! 对应 Java `me.chanjar.weixin.cp.bean.school.user.WxCpListParentResult.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::school::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpListParentResult {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "parents", default)]
    pub parents: Vec<Parent>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Parent {
    #[serde(rename = "parent_userid", default)]
    pub parent_user_id: String,
    #[serde(rename = "mobile", default)]
    pub mobile: String,
    #[serde(rename = "external_userid", default)]
    pub external_user_id: String,
    #[serde(rename = "is_subscribe", default)]
    pub is_subscribe: i32,
    #[serde(rename = "children", default)]
    pub children: Vec<crate::bean::school::user::wx_cp_list_parent_result::Children>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Children {
    #[serde(rename = "student_userid", default)]
    pub student_user_id: String,
    #[serde(rename = "relation", default)]
    pub relation: String,
    #[serde(rename = "name", default)]
    pub name: String,
}

impl WxCpListParentResult {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpListParentResult 解析失败: {e}"))
    }
}

impl WxCpListParentResult {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpListParentResult 序列化失败: {e}"))
    }
}
